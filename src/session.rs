use crate::diff::{DiffLine, LineKind, Side};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LineAnchor {
    pub repo: String,
    pub file: String,
    pub side: Side,
    pub old_line: Option<u32>,
    pub new_line: Option<u32>,
    pub hunk_header: String,
    pub line_text: String,
    pub context_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommentStatus {
    Draft,
    Copied,
    Resolved,
    Stale,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub anchor: LineAnchor,
    pub body: String,
    pub status: CommentStatus,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyBatch {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub scope: String,
    pub comment_ids: Vec<String>,
    pub prompt_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub workspace_root: String,
    pub cursor: usize,
    pub visited_hunks: BTreeSet<String>,
    pub comments: BTreeMap<String, Comment>,
    pub batches: Vec<CopyBatch>,
    pub show_resolved: bool,
}
impl LineAnchor {
    pub fn new(repo: &str, file: &str, hunk: &str, line: &DiffLine) -> Self {
        let side = match line.kind {
            LineKind::Add => Side::New,
            LineKind::Remove => Side::Old,
            LineKind::Context => Side::Both,
        };
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}",
            hunk,
            line.old_lineno.unwrap_or(0),
            line.text
        ));
        let context_hash = format!("{:x}", hasher.finalize());
        Self {
            repo: repo.into(),
            file: file.into(),
            side,
            old_line: line.old_lineno,
            new_line: line.new_lineno,
            hunk_header: hunk.into(),
            line_text: line.text.clone(),
            context_hash,
        }
    }
    pub fn key(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
impl Session {
    pub fn path_for(root: &Path) -> PathBuf {
        let key = {
            let mut h = Sha256::new();
            h.update(root.to_string_lossy().as_bytes());
            format!("{:x}.json", h.finalize())
        };
        let mut d = dirs::state_dir().unwrap_or_else(std::env::temp_dir);
        d.push("hawk-tui/sessions");
        let _ = fs::create_dir_all(&d);
        d.push(key);
        d
    }
    pub fn load(root: &Path) -> Self {
        let p = Self::path_for(root);
        fs::read_to_string(p)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| Self {
                workspace_root: root.to_string_lossy().into(),
                ..Default::default()
            })
    }
    pub fn save(&self, root: &Path) -> std::io::Result<()> {
        fs::write(
            Self::path_for(root),
            serde_json::to_vec_pretty(self).unwrap(),
        )
    }
    pub fn reset(root: &Path) {
        let _ = fs::remove_file(Self::path_for(root));
    }
    pub fn upsert_comment(&mut self, anchor: LineAnchor, body: String) {
        let k = anchor.key();
        if body.trim().is_empty() {
            self.comments.remove(&k);
        } else {
            let id = if let Some(c) = self.comments.get(&k) {
                c.id.clone()
            } else {
                format!("c{}", self.comments.len() + 1)
            };
            self.comments.insert(
                k,
                Comment {
                    id,
                    anchor,
                    body,
                    status: CommentStatus::Draft,
                },
            );
        }
    }
    pub fn toggle_resolved(&mut self, key: &str) {
        if let Some(c) = self.comments.get_mut(key) {
            c.status = if c.status == CommentStatus::Resolved {
                CommentStatus::Draft
            } else {
                CommentStatus::Resolved
            }
        }
    }
    pub fn visible_comments(&self) -> Vec<&Comment> {
        self.comments
            .values()
            .filter(|c| self.show_resolved || c.status != CommentStatus::Resolved)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_comment_and_persist() {
        let d = tempfile::tempdir().unwrap();
        let mut s = Session {
            workspace_root: "w".into(),
            ..Default::default()
        };
        let l = DiffLine {
            kind: LineKind::Add,
            old_lineno: None,
            new_lineno: Some(1),
            text: "x".into(),
        };
        let a = LineAnchor::new("r", "f", "@@", &l);
        s.upsert_comment(a.clone(), "a".into());
        s.upsert_comment(a, "b".into());
        assert_eq!(s.comments.len(), 1);
        s.save(d.path()).unwrap();
        assert_eq!(Session::load(d.path()).comments.len(), 1);
        Session::reset(d.path());
    }
}
