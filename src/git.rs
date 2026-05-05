use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOutput {
    pub stdout: String,
    pub stderr: String,
}

pub trait GitAdapter {
    fn run(&self, repo: &Path, args: &[&str]) -> Result<GitOutput, String>;
    fn repo_root(&self, dir: &Path) -> Option<PathBuf> {
        self.run(dir, &["rev-parse", "--show-toplevel"])
            .ok()
            .map(|o| PathBuf::from(o.stdout.trim()))
    }
    fn has_changes(&self, repo: &Path) -> bool {
        self.run(repo, &["status", "--porcelain"])
            .map(|o| !o.stdout.trim().is_empty())
            .unwrap_or(true)
    }
    fn diff(&self, repo: &Path, args: &[&str]) -> Result<String, String> {
        self.run(repo, args).map(|o| o.stdout)
    }
    fn untracked(&self, repo: &Path) -> Result<String, String> {
        self.run(repo, &["ls-files", "--others", "--exclude-standard", "-z"])
            .map(|o| o.stdout)
    }
    fn dirty_fingerprint_input(&self, repo: &Path) -> String {
        let mut s = String::new();
        if let Ok(o) = self.run(repo, &["status", "--porcelain=v1", "-z"]) {
            s.push_str(&o.stdout);
        }
        if let Ok(o) = self.run(repo, &["diff", "--no-ext-diff"]) {
            s.push_str(&o.stdout);
        }
        s
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CliGit;
impl GitAdapter for CliGit {
    fn run(&self, repo: &Path, args: &[&str]) -> Result<GitOutput, String> {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .map_err(|e| e.to_string())?;
        if out.status.success() {
            Ok(GitOutput {
                stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            })
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cli_git_reports_missing_repo_as_no_root() {
        assert!(CliGit.repo_root(Path::new("/")).is_none());
    }
}
