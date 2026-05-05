use crate::session::{Comment, CommentStatus, LineAnchor, Session};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SessionStore {
    pub(crate) root: PathBuf,
    pub session: Session,
    pub(crate) autosaves: usize,
}
impl SessionStore {
    pub fn load(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        let session = Session::load(&root);
        Self {
            root,
            session,
            autosaves: 0,
        }
    }
    pub fn save(&mut self) {
        let _ = self.session.save(&self.root);
        self.autosaves += 1;
    }
    pub fn autosaves(&self) -> usize {
        self.autosaves
    }
    pub fn reset(&mut self) {
        Session::reset(&self.root);
        self.session = Session {
            workspace_root: self.root.to_string_lossy().into(),
            ..Default::default()
        };
        self.save();
    }
    pub fn upsert_comment(&mut self, anchor: LineAnchor, body: String) {
        self.session.upsert_comment(anchor, body);
        self.save();
    }
    pub fn delete_anchor(&mut self, anchor: &LineAnchor) {
        self.session.comments.remove(&anchor.key());
        self.save();
    }
    pub fn delete_all_visible(&mut self) {
        let keys: Vec<_> = self
            .session
            .visible_comments()
            .into_iter()
            .map(|c| c.anchor.key())
            .collect();
        for k in keys {
            self.session.comments.remove(&k);
        }
        self.save();
    }
    pub fn toggle_resolved(&mut self, anchor: &LineAnchor) {
        self.session.toggle_resolved(&anchor.key());
        self.save();
    }
    pub fn set_show_resolved(&mut self, show: bool) {
        self.session.show_resolved = show;
        self.save();
    }
    pub fn clear_resolved(&mut self) {
        self.session
            .comments
            .retain(|_, c| c.status != CommentStatus::Resolved);
        self.save();
    }
    pub fn mark_copied(&mut self, ids: &[String], scope: &str, prompt: &str) {
        crate::prompt::mark_copied(&mut self.session, ids, scope, prompt);
        self.save();
    }
    pub fn reconcile_missing(&mut self, existing_keys: &std::collections::BTreeSet<String>) {
        for (k, c) in self.session.comments.iter_mut() {
            if !existing_keys.contains(k) {
                if c.status == CommentStatus::Copied {
                    c.status = CommentStatus::Resolved
                } else if c.status != CommentStatus::Draft {
                    c.status = CommentStatus::Stale
                }
            }
        }
        self.save();
    }
    pub fn visible_comments(&self) -> Vec<&Comment> {
        self.session.visible_comments()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{DiffLine, LineKind};
    #[test]
    fn store_autosaves_mutations() {
        let d = tempfile::tempdir().unwrap();
        let mut st = SessionStore::load(d.path());
        let l = DiffLine {
            kind: LineKind::Add,
            old_lineno: None,
            new_lineno: Some(1),
            text: "x".into(),
        };
        let a = LineAnchor::new("r", "f", "@@", &l);
        st.upsert_comment(a.clone(), "body".into());
        assert_eq!(st.autosaves(), 1);
        st.toggle_resolved(&a);
        assert_eq!(
            st.session.comments.values().next().unwrap().status,
            CommentStatus::Resolved
        );
        st.clear_resolved();
        assert!(st.session.comments.is_empty());
    }
}
