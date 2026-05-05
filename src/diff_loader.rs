use crate::{
    config::{Config, ReviewMode},
    diff::{self, DiffFile, FileStatus, RepoDiff},
    git::GitAdapter,
    text::TextStatus,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DiffLoadRequest<'a> {
    pub repo: &'a Path,
    pub display: String,
    pub mode: &'a ReviewMode,
    pub config: &'a Config,
    pub nested: &'a [PathBuf],
}

pub struct DiffLoader;
impl DiffLoader {
    pub fn load<G: GitAdapter>(git: &G, req: DiffLoadRequest<'_>) -> RepoDiff {
        let mut files = Vec::new();
        let args = diff_args(req.mode, req.config);
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        match git.diff(req.repo, &refs) {
            Ok(s) => files.extend(diff::parse_unified_diff(&s)),
            Err(e) => {
                return RepoDiff {
                    repo_path: req.repo.into(),
                    display_path: req.display,
                    files,
                    error: Some(e),
                }
            }
        }
        if matches!(req.mode, ReviewMode::Default) && req.config.include_untracked {
            if let Ok(list) = git.untracked(req.repo) {
                add_untracked(&mut files, req.repo, &list, req.config, req.nested);
            }
        }
        RepoDiff {
            repo_path: req.repo.into(),
            display_path: req.display,
            files,
            error: None,
        }
    }
}
fn diff_args(mode: &ReviewMode, cfg: &Config) -> Vec<String> {
    match mode {
        ReviewMode::Default => vec![
            "diff".into(),
            "--find-renames".into(),
            format!("-U{}", cfg.diff_context_lines),
        ],
        ReviewMode::Staged => vec![
            "diff".into(),
            "--cached".into(),
            "--find-renames".into(),
            format!("-U{}", cfg.diff_context_lines),
        ],
        ReviewMode::Base(b) => vec![
            "diff".into(),
            "--find-renames".into(),
            format!("{}...HEAD", b),
            format!("-U{}", cfg.diff_context_lines),
        ],
        ReviewMode::Ref(r) => vec![
            "diff".into(),
            "--find-renames".into(),
            r.clone(),
            format!("-U{}", cfg.diff_context_lines),
        ],
    }
}
fn add_untracked(
    files: &mut Vec<DiffFile>,
    repo: &Path,
    list: &str,
    cfg: &Config,
    nested: &[PathBuf],
) {
    for rel in list.split('\0').filter(|s| !s.is_empty()) {
        let p = repo.join(rel);
        if nested.iter().any(|n| p.starts_with(n)) {
            continue;
        }
        match crate::text::classify_path(&p, cfg) {
            TextStatus::Text => {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    let df = diff::new_file_diff(rel, &s);
                    let (added, removed) = df.counts();
                    if added + removed > cfg.eager_diff_lines {
                        files.push(DiffFile {
                            old_path: None,
                            new_path: Some(rel.into()),
                            status: FileStatus::Collapsed {
                                reason: "over eager diff-line threshold".into(),
                                added,
                                removed,
                            },
                            hunks: vec![],
                        })
                    } else {
                        files.push(df)
                    }
                }
            }
            TextStatus::Huge { reason } => files.push(DiffFile {
                old_path: None,
                new_path: Some(rel.into()),
                status: FileStatus::Collapsed {
                    reason,
                    added: 0,
                    removed: 0,
                },
                hunks: vec![],
            }),
            TextStatus::Binary(r) | TextStatus::TooLarge(r) => files.push(DiffFile {
                old_path: None,
                new_path: Some(rel.into()),
                status: FileStatus::Skipped { reason: r },
                hunks: vec![],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::{GitAdapter, GitOutput};
    use std::{collections::HashMap, path::Path};
    struct Fake {
        map: HashMap<String, Result<String, String>>,
    }
    impl GitAdapter for Fake {
        fn run(&self, _: &Path, args: &[&str]) -> Result<GitOutput, String> {
            let k = args.join(" ");
            self.map
                .get(&k)
                .cloned()
                .unwrap_or_else(|| Ok(String::new()))
                .map(|stdout| GitOutput {
                    stdout,
                    stderr: String::new(),
                })
        }
    }
    #[test]
    fn loader_includes_untracked_text() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("new.rs"), "fn main(){}\n").unwrap();
        let mut map = HashMap::new();
        map.insert("diff --find-renames -U3".into(), Ok(String::new()));
        map.insert(
            "ls-files --others --exclude-standard -z".into(),
            Ok("new.rs\0".into()),
        );
        let f = Fake { map };
        let cfg = Config::default();
        let r = DiffLoader::load(
            &f,
            DiffLoadRequest {
                repo: d.path(),
                display: ".".into(),
                mode: &ReviewMode::Default,
                config: &cfg,
                nested: &[],
            },
        );
        assert!(r.files.iter().any(|x| x.path() == "new.rs"));
    }
}
