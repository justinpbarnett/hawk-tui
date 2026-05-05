use crate::{
    diff::{DiffLine, LineKind, RepoDiff},
    session::{CommentStatus, LineAnchor, Session},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewRow {
    Repo(String),
    File {
        repo: String,
        path: String,
        added: usize,
        removed: usize,
    },
    Hunk {
        repo: String,
        file: String,
        header: String,
    },
    Line {
        repo: String,
        file: String,
        hunk: String,
        line: DiffLine,
    },
    Placeholder(String),
}

#[derive(Debug, Clone, Default)]
pub struct ReviewDocument {
    rows: Vec<ReviewRow>,
}

impl ReviewDocument {
    pub fn from_repos(repos: &[RepoDiff], session: &Session) -> Self {
        Self {
            rows: flatten(repos, session),
        }
    }
    pub fn rows(&self) -> &[ReviewRow] {
        &self.rows
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn row(&self, idx: usize) -> Option<&ReviewRow> {
        self.rows.get(idx)
    }
    pub fn anchor_at(&self, idx: usize) -> Option<LineAnchor> {
        self.row(idx).and_then(anchor_from_row)
    }
    pub fn next_changed_line_after(&self, cursor: usize) -> Option<usize> {
        let changes = self.changed_lines();
        changes
            .iter()
            .copied()
            .find(|i| *i > cursor)
            .or_else(|| changes.first().copied())
    }
    pub fn prev_changed_line_before(&self, cursor: usize) -> Option<usize> {
        let changes = self.changed_lines();
        changes
            .iter()
            .rev()
            .copied()
            .find(|i| *i < cursor)
            .or_else(|| changes.last().copied())
    }
    pub fn next_hunk_after(&self, cursor: usize) -> Option<usize> {
        let hs = self.hunks();
        hs.iter()
            .copied()
            .find(|i| *i > cursor)
            .or_else(|| hs.first().copied())
    }
    pub fn prev_hunk_before(&self, cursor: usize) -> Option<usize> {
        let hs = self.hunks();
        hs.iter()
            .rev()
            .copied()
            .find(|i| *i < cursor)
            .or_else(|| hs.last().copied())
    }
    pub fn next_visible_comment_after(&self, cursor: usize, session: &Session) -> Option<usize> {
        let idxs = self.visible_comment_rows(session);
        idxs.iter()
            .copied()
            .find(|i| *i > cursor)
            .or_else(|| idxs.first().copied())
    }
    pub fn prev_visible_comment_before(&self, cursor: usize, session: &Session) -> Option<usize> {
        let idxs = self.visible_comment_rows(session);
        idxs.iter()
            .rev()
            .copied()
            .find(|i| *i < cursor)
            .or_else(|| idxs.last().copied())
    }
    pub fn row_for_anchor(&self, anchor: &LineAnchor) -> Option<usize> {
        let key = anchor.key();
        self.rows
            .iter()
            .position(|r| anchor_from_row(r).map(|a| a.key()) == Some(key.clone()))
    }
    fn changed_lines(&self) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                matches!(
                    r,
                    ReviewRow::Line {
                        line: DiffLine {
                            kind: LineKind::Add | LineKind::Remove,
                            ..
                        },
                        ..
                    }
                )
            })
            .map(|(i, _)| i)
            .collect()
    }
    fn hunks(&self) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, r)| matches!(r, ReviewRow::Hunk { .. }))
            .map(|(i, _)| i)
            .collect()
    }
    fn visible_comment_rows(&self, session: &Session) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                let a = anchor_from_row(r)?;
                session.comments.get(&a.key()).and_then(|c| {
                    (c.status != CommentStatus::Resolved || session.show_resolved).then_some(i)
                })
            })
            .collect()
    }
}

pub fn anchor_from_row(r: &ReviewRow) -> Option<LineAnchor> {
    if let ReviewRow::Line {
        repo,
        file,
        hunk,
        line,
    } = r
    {
        Some(LineAnchor::new(repo, file, hunk, line))
    } else {
        None
    }
}

pub fn flatten(repos: &[RepoDiff], session: &Session) -> Vec<ReviewRow> {
    let mut rows = Vec::new();
    for r in repos {
        rows.push(ReviewRow::Repo(r.display_path.clone()));
        if let Some(e) = &r.error {
            rows.push(ReviewRow::Placeholder(format!("repo error: {e}")));
            continue;
        }
        for f in &r.files {
            let path = f.path();
            let (a, rm) = f.counts();
            rows.push(ReviewRow::File {
                repo: r.display_path.clone(),
                path: path.clone(),
                added: a,
                removed: rm,
            });
            if let crate::diff::FileStatus::Skipped { reason }
            | crate::diff::FileStatus::Collapsed { reason, .. } = &f.status
            {
                rows.push(ReviewRow::Placeholder(reason.clone()));
                continue;
            }
            for h in &f.hunks {
                rows.push(ReviewRow::Hunk {
                    repo: r.display_path.clone(),
                    file: path.clone(),
                    header: h.header.clone(),
                });
                for l in &h.lines {
                    let anchor = LineAnchor::new(&r.display_path, &path, &h.header, l);
                    if session.show_resolved
                        || session
                            .comments
                            .get(&anchor.key())
                            .map(|c| c.status != CommentStatus::Resolved)
                            .unwrap_or(true)
                    {
                        rows.push(ReviewRow::Line {
                            repo: r.display_path.clone(),
                            file: path.clone(),
                            hunk: h.header.clone(),
                            line: l.clone(),
                        });
                    }
                }
            }
        }
    }
    rows
}

pub fn line_is_content(row: &ReviewRow) -> bool {
    matches!(
        row,
        ReviewRow::Line {
            line: DiffLine {
                kind: LineKind::Add | LineKind::Remove | LineKind::Context,
                ..
            },
            ..
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{DiffFile, FileStatus, Hunk, LineKind};
    fn repo() -> RepoDiff {
        RepoDiff {
            repo_path: ".".into(),
            display_path: "r".into(),
            error: None,
            files: vec![DiffFile {
                old_path: Some("a".into()),
                new_path: Some("a".into()),
                status: FileStatus::Modified,
                hunks: vec![
                    Hunk {
                        header: "@@ 1".into(),
                        lines: vec![DiffLine {
                            kind: LineKind::Add,
                            old_lineno: None,
                            new_lineno: Some(1),
                            text: "x".into(),
                        }],
                    },
                    Hunk {
                        header: "@@ 2".into(),
                        lines: vec![DiffLine {
                            kind: LineKind::Add,
                            old_lineno: None,
                            new_lineno: Some(2),
                            text: "y".into(),
                        }],
                    },
                ],
            }],
        }
    }
    #[test]
    fn document_navigates_hunks_comments_and_changed_lines() {
        let mut s = Session::default();
        let d = ReviewDocument::from_repos(&[repo()], &s);
        assert_eq!(d.next_hunk_after(0), Some(2));
        assert_eq!(d.prev_hunk_before(0), Some(4));
        assert_eq!(d.next_changed_line_after(0), Some(3));
        assert_eq!(d.prev_changed_line_before(0), Some(5));
        let a = d.anchor_at(3).unwrap();
        s.upsert_comment(a, "note".into());
        let d = ReviewDocument::from_repos(&[repo()], &s);
        assert_eq!(d.next_visible_comment_after(0, &s), Some(3));
    }
}
