use crate::session::{CommentStatus, CopyBatch, Session};
use chrono::Utc;
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyScope {
    Uncopied,
    AllVisible,
}
pub fn build_prompt(session: &Session, scope: CopyScope) -> Option<(String, Vec<String>)> {
    let mut cs: Vec<_> = session
        .visible_comments()
        .into_iter()
        .filter(|c| scope == CopyScope::AllVisible || c.status == CommentStatus::Draft)
        .collect();
    if cs.is_empty() {
        return None;
    }
    cs.sort_by_key(|c| {
        (
            c.anchor.repo.clone(),
            c.anchor.file.clone(),
            c.anchor.old_line.or(c.anchor.new_line).unwrap_or(0),
        )
    });
    let mut out=String::from("Please address these local Hawk review comments. Keep changes focused and reply with a summary.\n");
    let (mut repo, mut file) = (String::new(), String::new());
    let mut ids = Vec::new();
    for c in cs {
        if c.anchor.repo != repo {
            repo = c.anchor.repo.clone();
            out.push_str(&format!("\n## Repo `{repo}`\n"));
            file.clear();
        }
        if c.anchor.file != file {
            file = c.anchor.file.clone();
            out.push_str(&format!("\n### `{file}`\n"));
        }
        ids.push(c.id.clone());
        let line = match c.anchor.side {
            crate::diff::Side::Old => {
                format!("old line {} (removed)", c.anchor.old_line.unwrap_or(0))
            }
            crate::diff::Side::New => {
                format!("new line {} (added)", c.anchor.new_line.unwrap_or(0))
            }
            crate::diff::Side::Both => format!(
                "line {} (context)",
                c.anchor.new_line.or(c.anchor.old_line).unwrap_or(0)
            ),
        };
        out.push_str(&format!(
            "- {} ({})\n  Hunk: `{}`\n  Context: `{}`\n  Comment:\n{}\n",
            file,
            line,
            c.anchor.hunk_header,
            c.anchor.line_text,
            format_comment_body(&c.body)
        ));
    }
    Some((out, ids))
}
fn format_comment_body(body: &str) -> String {
    body.trim_end()
        .lines()
        .map(|line| format!("    {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn mark_copied(session: &mut Session, ids: &[String], scope: &str, prompt: &str) {
    for c in session.comments.values_mut() {
        if ids.contains(&c.id) && c.status == CommentStatus::Draft {
            c.status = CommentStatus::Copied
        }
    }
    let mut h = Sha256::new();
    h.update(prompt);
    session.batches.push(CopyBatch {
        id: format!("batch-{}", session.batches.len() + 1),
        timestamp: Utc::now(),
        scope: scope.into(),
        comment_ids: ids.to_vec(),
        prompt_hash: format!("{:x}", h.finalize()),
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diff::{DiffLine, LineKind},
        session::{LineAnchor, Session},
    };
    #[test]
    fn multiline_comments_render_as_an_indented_block_without_trailing_blank_lines() {
        let mut s = Session::default();
        let l = DiffLine {
            kind: LineKind::Add,
            old_lineno: None,
            new_lineno: Some(3),
            text: "new".into(),
        };
        s.upsert_comment(
            LineAnchor::new("repo", "new.rs", "@@ -1 +1", &l),
            "abcd\nefg\n\n".into(),
        );

        let (p, _) = build_prompt(&s, CopyScope::Uncopied).unwrap();

        assert!(p.contains("  Comment:\n    abcd\n    efg\n"));
        assert!(!p.contains("efg\n    \n"));
    }

    #[test]
    fn prompt_group_and_mark() {
        let mut s = Session::default();
        let l = DiffLine {
            kind: LineKind::Remove,
            old_lineno: Some(2),
            new_lineno: None,
            text: "bad".into(),
        };
        s.upsert_comment(
            LineAnchor::new("repo", "old.rs", "@@ -2", &l),
            "why?".into(),
        );
        let (p, ids) = build_prompt(&s, CopyScope::Uncopied).unwrap();
        assert!(p.contains("old line 2 (removed)"));
        mark_copied(&mut s, &ids, "uncopied", &p);
        assert_eq!(s.batches.len(), 1);
        assert!(build_prompt(&s, CopyScope::Uncopied).is_none());
    }
}
