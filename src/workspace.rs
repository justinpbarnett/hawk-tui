use crate::{
    config::Config,
    git::{CliGit, GitAdapter},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoTarget {
    pub root: PathBuf,
    pub display: String,
    pub nested_children: Vec<PathBuf>,
}

pub fn discover(cwd: &Path, cfg: &Config, force_repo: bool, force_ws: bool) -> Vec<RepoTarget> {
    discover_with_git(&CliGit, cwd, cfg, force_repo, force_ws)
}

pub fn discover_with_git<G: GitAdapter>(
    git: &G,
    cwd: &Path,
    cfg: &Config,
    force_repo: bool,
    force_ws: bool,
) -> Vec<RepoTarget> {
    let mut roots = BTreeSet::new();
    if !force_ws {
        if let Some(r) = git.repo_root(cwd) {
            roots.insert(r);
            if force_repo {
                return targets(roots, cwd);
            }
        }
    }
    if force_ws || !force_repo {
        for e in WalkDir::new(cwd)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                !cfg.workspace_excludes
                    .iter()
                    .any(|x| e.file_name().to_string_lossy() == x.as_str())
            })
            .filter_map(Result::ok)
        {
            if e.file_type().is_dir() && e.path().join(".git").exists() {
                roots.insert(
                    e.path()
                        .canonicalize()
                        .unwrap_or_else(|_| e.path().to_path_buf()),
                );
            }
        }
    }
    let mut ts = targets(roots, cwd);
    ts.retain(|t| git.has_changes(&t.root));
    ts
}
fn targets(roots: BTreeSet<PathBuf>, cwd: &Path) -> Vec<RepoTarget> {
    let all: Vec<_> = roots.into_iter().collect();
    all.iter()
        .map(|r| {
            let children = all
                .iter()
                .filter(|c| *c != r && c.starts_with(r))
                .cloned()
                .collect();
            RepoTarget {
                root: r.clone(),
                display: r
                    .strip_prefix(cwd)
                    .unwrap_or(r)
                    .to_string_lossy()
                    .trim_start_matches('/')
                    .to_string(),
                nested_children: children,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    fn git(p: &Path, a: &[&str]) {
        assert!(Command::new("git")
            .args(a)
            .current_dir(p)
            .output()
            .unwrap()
            .status
            .success())
    }
    #[test]
    fn discovers_nested_and_skips() {
        let d = tempfile::tempdir().unwrap();
        let child = d.path().join("client");
        std::fs::create_dir(&child).unwrap();
        git(d.path(), &["init"]);
        git(&child, &["init"]);
        std::fs::write(child.join("a"), "x").unwrap();
        git(&child, &["add", "."]);
        let cfg = Config::default();
        let r = discover(d.path(), &cfg, false, true);
        assert!(r.iter().any(|x| x.root.ends_with("client")));
    }
}
