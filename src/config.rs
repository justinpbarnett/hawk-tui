use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug, Clone)]
#[command(name = "hawk", about = "Local keyboard-first code review TUI")]
pub struct Cli {
    #[arg(long)]
    pub staged: bool,
    #[arg(long)]
    pub base: Option<String>,
    #[arg(long = "ref")]
    pub diff_ref: Option<String>,
    #[arg(long)]
    pub repo: bool,
    #[arg(long)]
    pub workspace: bool,
    #[arg(long)]
    pub no_tui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub diff_context_lines: usize,
    pub eager_file_size: u64,
    pub eager_diff_lines: usize,
    pub absolute_file_size: u64,
    pub include_untracked: bool,
    pub workspace_excludes: Vec<String>,
    pub clipboard_preference: Option<String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            diff_context_lines: 3,
            eager_file_size: 512 * 1024,
            eager_diff_lines: 3000,
            absolute_file_size: 5 * 1024 * 1024,
            include_untracked: true,
            workspace_excludes: vec![
                "node_modules".into(),
                "target".into(),
                ".git".into(),
                "dist".into(),
                "build".into(),
                "vendor".into(),
            ],
            clipboard_preference: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut c = Self::default();
        if let Some(mut p) = dirs::config_dir() {
            p.push("hawk-tui/config.toml");
            if let Ok(s) = fs::read_to_string(p) {
                if let Ok(file) = toml::from_str::<Config>(&s) {
                    c = file;
                }
            }
        }
        c
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReviewMode {
    Default,
    Staged,
    Base(String),
    Ref(String),
}
#[derive(Debug, Clone)]
pub struct Options {
    pub mode: ReviewMode,
    pub force_repo: bool,
    pub force_workspace: bool,
    pub cwd: PathBuf,
    pub config: Config,
}
impl Options {
    pub fn from_cli_at(cli: Cli, cwd: PathBuf) -> Self {
        let mode = if cli.staged {
            ReviewMode::Staged
        } else if let Some(b) = cli.base {
            ReviewMode::Base(b)
        } else if let Some(r) = cli.diff_ref {
            ReviewMode::Ref(r)
        } else {
            ReviewMode::Default
        };
        Self {
            mode,
            force_repo: cli.repo,
            force_workspace: cli.workspace,
            cwd,
            config: Config::load(),
        }
    }
}
