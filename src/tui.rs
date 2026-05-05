use crate::{
    commands::{AppState, CommandHandler},
    document::ReviewRow,
    engine::ReviewEngine,
};
use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};
use std::io;

pub use crate::commands::Mode;

pub fn run(mut engine: ReviewEngine) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let mut app = AppState::default();
    let res = loop {
        engine.poll_dirty();
        term.draw(|f| draw(f, &engine, &app))?;
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(k) = event::read()? {
                CommandHandler::handle(&mut app, k, &mut engine);
                if app.quit {
                    break Ok(());
                }
            }
        }
    };
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    res
}

fn draw(f: &mut ratatui::Frame, engine: &ReviewEngine, app: &AppState) {
    let constraints = vec![Constraint::Min(1), Constraint::Length(1)];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.size());
    let items: Vec<_> = engine
        .document
        .rows()
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let row = if let Mode::Editing(buffer) = &app.mode {
                if i == app.cursor {
                    vec![row_line(r), editing_ghost_line(buffer)]
                } else {
                    vec![row_line(r)]
                }
            } else {
                vec![row_line(r)]
            };
            ListItem::new(row).style(if i == app.cursor {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            })
        })
        .collect();
    if matches!(app.mode, Mode::Help) {
        f.render_widget(help_panel(), chunks[0]);
    } else if app.sidebar {
        f.render_widget(sidebar_panel(engine), chunks[0]);
    } else if matches!(app.mode, Mode::CommentList { .. }) {
        f.render_widget(comment_list_panel(engine), chunks[0]);
    } else {
        f.render_widget(
            List::new(items).block(Block::default().title("hawk").borders(Borders::ALL)),
            chunks[0],
        );
    }
    let status_index = 1;
    let dirty = if engine.dirty {
        " | DIRTY: press r to reload"
    } else {
        ""
    };
    f.render_widget(
        Paragraph::new(format!(
            "{} rows{dirty} | ? help | {}",
            engine.document.len(),
            app.status
        )),
        chunks[status_index],
    );
}
fn editing_ghost_line(buffer: &str) -> Line<'static> {
    let visible = if buffer.is_empty() {
        "  💬 Type comment here. Esc saves."
    } else {
        return Line::from(vec![Span::styled(
            format!("  💬 {buffer}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )]);
    };
    Line::from(vec![Span::styled(
        visible,
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )])
}

fn sidebar_panel(engine: &ReviewEngine) -> Paragraph<'static> {
    let mut out = String::from("Files\n\n");
    for r in engine.document.rows() {
        if let ReviewRow::File {
            path,
            added,
            removed,
            ..
        } = r
        {
            out.push_str(&format!("{path}  +{added} -{removed}\n"));
        }
    }
    Paragraph::new(out)
        .block(
            Block::default()
                .title("File sidebar (e closes)")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false })
}

fn comment_list_panel(engine: &ReviewEngine) -> Paragraph<'static> {
    let mut out = String::from("Comments\n\n");
    for c in engine.session.visible_comments() {
        out.push_str(&format!(
            "{}:{} — {}\n",
            c.anchor.file,
            c.anchor.new_line.or(c.anchor.old_line).unwrap_or(0),
            c.body.replace('\n', " ")
        ));
    }
    if engine.session.visible_comments().is_empty() {
        out.push_str("No visible comments.\n");
    }
    Paragraph::new(out)
        .block(
            Block::default()
                .title("Comment list (Esc closes)")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false })
}

fn help_panel() -> Paragraph<'static> {
    Paragraph::new(
        "Hawk help\n\n\
Navigation:\n\
  j/k        move line down/up\n\
  J/K, Tab   jump between hunks\n\
  e          toggle file sidebar\n\
  c          open comment list\n\
\nComments:\n\
  o          add/edit comment on current diff line\n\
  x          delete current line comment\n\
  X          delete visible comments with confirmation\n\
  n / p,N    next / previous unresolved comment\n\
  m          toggle resolved\n\
  s          show/hide resolved\n\
\nExport and commands:\n\
  y / :w     copy uncopied comments\n\
  Y / :w!    copy all visible comments\n\
  r / :reload reload diff\n\
  q / :q     quit\n\
  :q!        force quit\n\
  ? or Esc   close help",
    )
    .block(Block::default().title("Hawk help").borders(Borders::ALL))
    .wrap(Wrap { trim: false })
}

fn row_line(r: &ReviewRow) -> Line<'static> {
    match r {
        ReviewRow::Repo(s) => Line::from(vec![Span::styled(
            format!("repo {s}"),
            Style::default().fg(Color::Cyan),
        )]),
        ReviewRow::File {
            path,
            added,
            removed,
            ..
        } => Line::from(format!("file {path} +{added} -{removed}")),
        ReviewRow::Hunk { header, .. } => Line::from(header.clone()),
        ReviewRow::Line { line, .. } => {
            let (p, c) = match line.kind {
                crate::diff::LineKind::Add => ("+", Color::Green),
                crate::diff::LineKind::Remove => ("-", Color::Red),
                crate::diff::LineKind::Context => (" ", Color::Gray),
            };
            Line::from(vec![Span::styled(
                format!("{}{}", p, line.text),
                Style::default().fg(c),
            )])
        }
        ReviewRow::CommentGhost { body } => Line::from(vec![Span::styled(
            format!("  💬 {}", body.replace('\n', " ⏎ ")),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )]),
        ReviewRow::Placeholder(s) => Line::from(format!("! {s}")),
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
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::backend::TestBackend;
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
    fn nav_help_command() {
        let mut e = eng();
        let mut a = AppState::default();
        CommandHandler::handle(&mut a, KeyEvent::from(KeyCode::Char('J')), &mut e);
        assert!(matches!(
            e.document.row(a.cursor),
            Some(ReviewRow::Hunk { .. })
        ));
        CommandHandler::handle(&mut a, KeyEvent::from(KeyCode::Char('?')), &mut e);
        assert_eq!(a.mode, Mode::Help);
    }

    #[test]
    fn help_mode_renders_keybindings() {
        let e = eng();
        let mut a = AppState::default();
        a.mode = Mode::Help;
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| draw(f, &e, &a)).unwrap();

        let rendered = format!("{:?}", terminal.backend().buffer());
        assert!(rendered.contains("Hawk help"));
        assert!(rendered.contains("j/k"));
        assert!(rendered.contains("copy uncopied comments"));
    }

    #[test]
    fn edit_mode_renders_comment_buffer_inline_below_current_line() {
        let e = eng();
        let mut a = AppState::default();
        a.mode = Mode::Editing("please fix this".into());
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| draw(f, &e, &a)).unwrap();

        let rendered = format!("{:?}", terminal.backend().buffer());
        assert!(rendered.contains("💬"));
        assert!(rendered.contains("please fix this"));
    }
}
