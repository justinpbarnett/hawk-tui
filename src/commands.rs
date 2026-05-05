use crate::{clipboard::FallbackClipboard, engine::ReviewEngine, prompt::CopyScope};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Nav,
    Help,
    Command(String),
    Editing(String),
    ConfirmDeleteAll,
    CommentList { selected: usize },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub cursor: usize,
    pub mode: Mode,
    pub sidebar: bool,
    pub status: String,
    pub quit: bool,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            cursor: 0,
            mode: Mode::Nav,
            sidebar: false,
            status: String::new(),
            quit: false,
        }
    }
}

pub struct CommandHandler;
impl CommandHandler {
    pub fn handle(app: &mut AppState, key: KeyEvent, engine: &mut ReviewEngine) {
        match &mut app.mode {
            Mode::Nav => match key.code {
                KeyCode::Char('q') => {
                    if engine.has_uncopied_drafts() {
                        app.status = "uncopied draft comments; use :q! to quit".into()
                    } else {
                        app.quit = true
                    }
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.quit = true
                }
                KeyCode::Char('j') => {
                    app.cursor = (app.cursor + 1).min(engine.document.len().saturating_sub(1))
                }
                KeyCode::Char('k') => app.cursor = app.cursor.saturating_sub(1),
                KeyCode::Char('J') | KeyCode::Tab => {
                    if let Some(i) = engine.document.next_hunk_after(app.cursor) {
                        app.cursor = i
                    }
                }
                KeyCode::Char('K') => {
                    if let Some(i) = engine.document.prev_hunk_before(app.cursor) {
                        app.cursor = i
                    }
                }
                KeyCode::Char('?') => app.mode = Mode::Help,
                KeyCode::Char(':') => app.mode = Mode::Command(String::new()),
                KeyCode::Char('e') => app.sidebar = !app.sidebar,
                KeyCode::Char('o') => {
                    if let Some(anchor) = engine.document.anchor_at(app.cursor) {
                        let existing = engine
                            .session
                            .comments
                            .get(&anchor.key())
                            .map(|c| c.body.clone())
                            .unwrap_or_default();
                        app.mode = Mode::Editing(existing)
                    } else {
                        app.status = "comments attach only to diff content lines".into()
                    }
                }
                KeyCode::Char('x') => engine.delete_comment(app.cursor),
                KeyCode::Char('X') => app.mode = Mode::ConfirmDeleteAll,
                KeyCode::Char('y') => {
                    app.status = engine.export(CopyScope::Uncopied, &mut FallbackClipboard)
                }
                KeyCode::Char('Y') => {
                    app.status = engine.export(CopyScope::AllVisible, &mut FallbackClipboard)
                }
                KeyCode::Char('r') => {
                    engine.reload();
                    app.status = "reloaded".into()
                }
                KeyCode::Char('m') => engine.toggle_resolved(app.cursor),
                KeyCode::Char('s') => engine.toggle_show_resolved(),
                KeyCode::Char('n') => {
                    if let Some(i) = engine
                        .document
                        .next_visible_comment_after(app.cursor, &engine.session)
                    {
                        app.cursor = i
                    }
                }
                KeyCode::Char('p') | KeyCode::Char('N') => {
                    if let Some(i) = engine
                        .document
                        .prev_visible_comment_before(app.cursor, &engine.session)
                    {
                        app.cursor = i
                    }
                }
                KeyCode::Char('c') => app.mode = Mode::CommentList { selected: 0 },
                _ => {}
            },
            Mode::Help => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                    app.mode = Mode::Nav
                }
            }
            Mode::Command(cmd) => match key.code {
                KeyCode::Enter => {
                    let c = cmd.clone();
                    app.mode = Mode::Nav;
                    run_command(app, &c, engine)
                }
                KeyCode::Esc => app.mode = Mode::Nav,
                KeyCode::Backspace => {
                    cmd.pop();
                }
                KeyCode::Char(ch) => cmd.push(ch),
                _ => {}
            },
            Mode::Editing(buf) => match key.code {
                KeyCode::Esc | KeyCode::Char('c')
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    let b = buf.clone();
                    engine.comment(app.cursor, b);
                    app.mode = Mode::Nav
                }
                KeyCode::Enter => buf.push('\n'),
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Char(ch) => buf.push(ch),
                _ => {}
            },
            Mode::ConfirmDeleteAll => match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    engine.delete_all_visible_comments();
                    app.mode = Mode::Nav
                }
                KeyCode::Esc | KeyCode::Char('n') => app.mode = Mode::Nav,
                _ => {}
            },
            Mode::CommentList { selected } => match key.code {
                KeyCode::Esc => app.mode = Mode::Nav,
                KeyCode::Char('j') => {
                    *selected = (*selected + 1)
                        .min(engine.session.visible_comments().len().saturating_sub(1))
                }
                KeyCode::Char('k') => *selected = selected.saturating_sub(1),
                KeyCode::Enter => {
                    if let Some(c) = engine.session.visible_comments().get(*selected) {
                        if let Some(i) = engine.document.row_for_anchor(&c.anchor) {
                            app.cursor = i
                        }
                    }
                    app.mode = Mode::Nav
                }
                _ => {}
            },
        }
    }
}
fn run_command(app: &mut AppState, c: &str, e: &mut ReviewEngine) {
    match c {
        "q" => app.quit = true,
        "q!" => app.quit = true,
        "w" => app.status = e.export(CopyScope::Uncopied, &mut FallbackClipboard),
        "w!" => app.status = e.export(CopyScope::AllVisible, &mut FallbackClipboard),
        "reload" => {
            e.reload();
            app.status = "reloaded".into()
        }
        "reset" => {
            e.reset_session();
            app.status = "session reset".into()
        }
        "clear-resolved" => e.clear_resolved(),
        _ => app.status = format!("unknown command: {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{Config, Options, ReviewMode},
        diff::{DiffFile, DiffLine, FileStatus, Hunk, LineKind, RepoDiff},
        document::ReviewDocument,
        session::Session,
    };
    fn eng() -> ReviewEngine {
        let repo = RepoDiff {
            repo_path: ".".into(),
            display_path: ".".into(),
            error: None,
            files: vec![DiffFile {
                old_path: Some("a".into()),
                new_path: Some("a".into()),
                status: FileStatus::Modified,
                hunks: vec![Hunk {
                    header: "@@ -1 +1".into(),
                    lines: vec![DiffLine {
                        kind: LineKind::Context,
                        old_lineno: Some(1),
                        new_lineno: Some(1),
                        text: "x".into(),
                    }],
                }],
            }],
        };
        let session = Session::default();
        let document = ReviewDocument::from_repos(&[repo.clone()], &session);
        ReviewEngine {
            opts: Options {
                mode: ReviewMode::Default,
                force_repo: true,
                force_workspace: false,
                cwd: ".".into(),
                config: Config::default(),
            },
            repos: vec![repo],
            document,
            session,
            dirty: false,
            fingerprint: String::new(),
        }
    }
    #[test]
    fn command_handler_navigates_and_opens_help() {
        let mut e = eng();
        let mut a = AppState::default();
        CommandHandler::handle(&mut a, KeyEvent::from(KeyCode::Char('J')), &mut e);
        assert!(matches!(
            e.document.row(a.cursor),
            Some(crate::document::ReviewRow::Hunk { .. })
        ));
        CommandHandler::handle(&mut a, KeyEvent::from(KeyCode::Char('?')), &mut e);
        assert_eq!(a.mode, Mode::Help);
    }
}
