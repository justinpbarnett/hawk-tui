use crate::config::{Config, ReviewMode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LineKind {
    Add,
    Remove,
    Context,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    Old,
    New,
    Both,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DiffLine {
    pub kind: LineKind,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hunk {
    pub header: String,
    pub lines: Vec<DiffLine>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed {
        from: String,
        to: String,
    },
    Skipped {
        reason: String,
    },
    Collapsed {
        reason: String,
        added: usize,
        removed: usize,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffFile {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub status: FileStatus,
    pub hunks: Vec<Hunk>,
}
impl DiffFile {
    pub fn path(&self) -> String {
        self.new_path
            .clone()
            .or(self.old_path.clone())
            .unwrap_or_default()
    }
    pub fn counts(&self) -> (usize, usize) {
        self.hunks
            .iter()
            .flat_map(|h| &h.lines)
            .fold((0, 0), |(a, r), l| match l.kind {
                LineKind::Add => (a + 1, r),
                LineKind::Remove => (a, r + 1),
                _ => (a, r),
            })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoDiff {
    pub repo_path: PathBuf,
    pub display_path: String,
    pub files: Vec<DiffFile>,
    pub error: Option<String>,
}

pub fn load_repo_diff(
    repo: &Path,
    display: String,
    mode: &ReviewMode,
    cfg: &Config,
    nested: &[PathBuf],
) -> RepoDiff {
    crate::diff_loader::DiffLoader::load(
        &crate::git::CliGit,
        crate::diff_loader::DiffLoadRequest {
            repo,
            display,
            mode,
            config: cfg,
            nested,
        },
    )
}

pub fn new_file_diff(path: &str, content: &str) -> DiffFile {
    let lines = content
        .lines()
        .enumerate()
        .map(|(i, l)| DiffLine {
            kind: LineKind::Add,
            old_lineno: None,
            new_lineno: Some((i + 1) as u32),
            text: l.into(),
        })
        .collect();
    DiffFile {
        old_path: None,
        new_path: Some(path.into()),
        status: FileStatus::Added,
        hunks: vec![Hunk {
            header: "@@ -0,0 +1 @@".into(),
            lines,
        }],
    }
}

pub fn parse_unified_diff(input: &str) -> Vec<DiffFile> {
    let mut files = Vec::new();
    let mut cur: Option<DiffFile> = None;
    let mut hunk: Option<Hunk> = None;
    let (mut old_ln, mut new_ln) = (0u32, 0u32);
    let (mut renamed_from, mut renamed_to) = (None::<String>, None::<String>);
    for line in input.lines() {
        if line.starts_with("diff --git ") {
            if let Some(h) = hunk.take() {
                if let Some(f) = cur.as_mut() {
                    f.hunks.push(h)
                }
            }
            if let Some(mut f) = cur.take() {
                if let (Some(a), Some(b)) = (renamed_from.take(), renamed_to.take()) {
                    f.status = FileStatus::Renamed { from: a, to: b }
                }
                files.push(f)
            }
            cur = Some(DiffFile {
                old_path: None,
                new_path: None,
                status: FileStatus::Modified,
                hunks: vec![],
            });
        } else if let Some(rest) = line.strip_prefix("rename from ") {
            renamed_from = Some(rest.into());
        } else if let Some(rest) = line.strip_prefix("rename to ") {
            renamed_to = Some(rest.into());
        } else if line.starts_with("deleted file mode") {
            if let Some(f) = cur.as_mut() {
                f.status = FileStatus::Deleted
            }
        } else if line.starts_with("new file mode") {
            if let Some(f) = cur.as_mut() {
                f.status = FileStatus::Added
            }
        } else if let Some(p) = line.strip_prefix("--- ") {
            let oldp = clean_path(p);
            if let Some(f) = cur.as_mut() {
                f.old_path = if p == "/dev/null" { None } else { Some(oldp) };
            }
        } else if let Some(p) = line.strip_prefix("+++ ") {
            let newp = clean_path(p);
            if let Some(f) = cur.as_mut() {
                f.new_path = if p == "/dev/null" { None } else { Some(newp) };
            }
        } else if line.starts_with("@@ ") {
            if let Some(h) = hunk.take() {
                if let Some(f) = cur.as_mut() {
                    f.hunks.push(h)
                }
            }
            let (o, n) = parse_hunk_lnos(line);
            old_ln = o;
            new_ln = n;
            hunk = Some(Hunk {
                header: line.into(),
                lines: vec![],
            });
        } else if let Some(h) = hunk.as_mut() {
            if line.starts_with("\\ No newline") {
                continue;
            }
            let (kind, text) = match line.chars().next() {
                Some('+') => (LineKind::Add, &line[1..]),
                Some('-') => (LineKind::Remove, &line[1..]),
                Some(' ') => (LineKind::Context, &line[1..]),
                _ => (LineKind::Context, line),
            };
            let dl = match kind {
                LineKind::Add => {
                    let l = DiffLine {
                        kind,
                        old_lineno: None,
                        new_lineno: Some(new_ln),
                        text: text.into(),
                    };
                    new_ln += 1;
                    l
                }
                LineKind::Remove => {
                    let l = DiffLine {
                        kind,
                        old_lineno: Some(old_ln),
                        new_lineno: None,
                        text: text.into(),
                    };
                    old_ln += 1;
                    l
                }
                LineKind::Context => {
                    let l = DiffLine {
                        kind,
                        old_lineno: Some(old_ln),
                        new_lineno: Some(new_ln),
                        text: text.into(),
                    };
                    old_ln += 1;
                    new_ln += 1;
                    l
                }
            };
            h.lines.push(dl);
        }
    }
    if let Some(h) = hunk.take() {
        if let Some(f) = cur.as_mut() {
            f.hunks.push(h)
        }
    }
    if let Some(mut f) = cur.take() {
        if let (Some(a), Some(b)) = (renamed_from.take(), renamed_to.take()) {
            f.status = FileStatus::Renamed { from: a, to: b }
        }
        files.push(f)
    }
    files
}
fn clean_path(p: &str) -> String {
    p.trim()
        .trim_start_matches("a/")
        .trim_start_matches("b/")
        .to_string()
}
fn parse_hunk_lnos(h: &str) -> (u32, u32) {
    let mut old = 1;
    let mut new = 1;
    for part in h.split_whitespace() {
        if let Some(s) = part.strip_prefix('-') {
            old = s.split(',').next().unwrap_or("1").parse().unwrap_or(1)
        }
        if let Some(s) = part.strip_prefix('+') {
            new = s.split(',').next().unwrap_or("1").parse().unwrap_or(1)
        }
    }
    (old, new)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_lines_and_rename() {
        let d="diff --git a/a b/b\nrename from a\nrename to b\n--- a/a\n+++ b/b\n@@ -1,2 +1,2 @@\n old\n-rm\n+add\n\\ No newline at end of file\n";
        let f = parse_unified_diff(d);
        assert_eq!(f[0].hunks[0].lines[1].old_lineno, Some(2));
        assert_eq!(f[0].hunks[0].lines[2].new_lineno, Some(2));
        assert!(matches!(f[0].status, FileStatus::Renamed { .. }));
    }
}
