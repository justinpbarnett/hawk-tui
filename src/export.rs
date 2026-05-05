use crate::{
    clipboard::Clipboard,
    prompt::{self, CopyScope},
    session_store::SessionStore,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportStatus {
    NoComments,
    Copied(String),
    Failed(String),
}
impl std::fmt::Display for ExportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoComments => write!(f, "no comments to copy"),
            Self::Copied(d) => write!(f, "copied to {d}"),
            Self::Failed(e) => write!(f, "copy failed: {e}"),
        }
    }
}

pub struct PromptExporter;
impl PromptExporter {
    pub fn export<C: Clipboard>(
        store: &mut SessionStore,
        scope: CopyScope,
        cb: &mut C,
    ) -> ExportStatus {
        if let Some((p, ids)) = prompt::build_prompt(&store.session, scope) {
            match cb.write(&p) {
                Ok(dest) => {
                    store.mark_copied(
                        &ids,
                        if scope == CopyScope::Uncopied {
                            "uncopied"
                        } else {
                            "all"
                        },
                        &p,
                    );
                    ExportStatus::Copied(dest)
                }
                Err(e) => ExportStatus::Failed(e),
            }
        } else {
            ExportStatus::NoComments
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        clipboard::Clipboard,
        diff::{DiffLine, LineKind},
        session::LineAnchor,
    };
    struct Fake {
        fail: bool,
        writes: usize,
    }
    impl Clipboard for Fake {
        fn write(&mut self, _: &str) -> Result<String, String> {
            if self.fail {
                Err("boom".into())
            } else {
                self.writes += 1;
                Ok("fake".into())
            }
        }
    }
    #[test]
    fn export_mutates_only_after_success() {
        let d = tempfile::tempdir().unwrap();
        let mut st = SessionStore::load(d.path());
        let l = DiffLine {
            kind: LineKind::Add,
            old_lineno: None,
            new_lineno: Some(1),
            text: "x".into(),
        };
        st.upsert_comment(LineAnchor::new("r", "f", "@@", &l), "note".into());
        let mut fail = Fake {
            fail: true,
            writes: 0,
        };
        assert!(matches!(
            PromptExporter::export(&mut st, CopyScope::Uncopied, &mut fail),
            ExportStatus::Failed(_)
        ));
        assert_eq!(st.session.batches.len(), 0);
        let mut ok = Fake {
            fail: false,
            writes: 0,
        };
        assert!(matches!(
            PromptExporter::export(&mut st, CopyScope::Uncopied, &mut ok),
            ExportStatus::Copied(_)
        ));
        assert_eq!(st.session.batches.len(), 1);
    }
}
