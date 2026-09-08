use crate::parser;
use crate::renderer;
use ansi_to_tui::IntoText;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::fs;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct TuiApp {
    file_path: Option<PathBuf>,
    raw_content: String,
    rendered_diagram: String,
    scroll_x: u16,
    scroll_y: u16,
    error_msg: Option<String>,
}

impl TuiApp {
    pub fn new(file_path: Option<PathBuf>, initial_content: String) -> Self {
        let mut app = Self {
            file_path,
            raw_content: initial_content,
            rendered_diagram: String::new(),
            scroll_x: 0,
            scroll_y: 0,
            error_msg: None,
        };
        app.recompute();
        app
    }

    pub fn reload_file(&mut self) {
        if let Some(ref path) = self.file_path {
            if let Ok(content) = fs::read_to_string(path) {
                self.raw_content = content;
                self.recompute();
            }
        }
    }

    pub fn recompute(&mut self) {
        let content = &self.raw_content;
        let diagram_to_parse = if content.contains("```mermaid") {
            let blocks = crate::markdown::extract_mermaid_blocks(content);
            blocks.first().cloned().unwrap_or_else(|| content.clone())
        } else {
            content.clone()
        };

        match parser::parse_mermaid(&diagram_to_parse) {
            Ok(diagram) => {
                self.rendered_diagram = renderer::render_ascii(&diagram);
                self.error_msg = None;
            }
            Err(e) => {
                self.error_msg = Some(e);
            }
        }
    }
}

pub fn run_tui(file_path: Option<PathBuf>, initial_content: String) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TuiApp::new(file_path.clone(), initial_content);

    let (tx, rx) = channel();
    let mut _watcher: Option<RecommendedWatcher> = None;

    if let Some(ref path) = file_path {
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        if let Some(parent) = path.parent() {
            let _ = watcher.watch(parent, RecursiveMode::NonRecursive);
        }
        let _ = watcher.watch(path, RecursiveMode::NonRecursive);
        _watcher = Some(watcher);
    }

    let res = run_loop(&mut terminal, &mut app, &rx);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("TUI Error: {:?}", err);
    }

    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut TuiApp,
    rx: &std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Ok(Ok(event)) = rx.try_recv() {
            if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                app.reload_file();
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let title = if let Some(ref p) = app.file_path {
                format!(" Mermaid TUI Viewer - {} ", p.display())
            } else {
                " Mermaid TUI Viewer (stdin) ".to_string()
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    title,
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ));

            let display_text: Text = if let Some(ref err) = app.error_msg {
                Text::from(format!("Diagram Parse Error:\n{}", err))
            } else {
                app.rendered_diagram
                    .as_bytes()
                    .into_text()
                    .unwrap_or_else(|_| Text::from(app.rendered_diagram.clone()))
            };

            let paragraph = Paragraph::new(display_text)
                .block(block)
                .scroll((app.scroll_y, app.scroll_x));

            f.render_widget(paragraph, chunks[0]);

            let status_line = Line::from(vec![
                Span::styled(" [q] ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Quit  "),
                Span::styled(" [hjkl / Arrows] ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Pan  "),
                Span::styled(" [0] ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Reset  "),
                Span::styled(" [r] ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Reload  "),
                Span::styled(format!(" (Pos: {}, {}) ", app.scroll_x, app.scroll_y), Style::default().fg(Color::DarkGray)),
            ]);

            f.render_widget(Paragraph::new(status_line), chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Char('h') | KeyCode::Left => {
                        app.scroll_x = app.scroll_x.saturating_sub(4);
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        app.scroll_x = app.scroll_x.saturating_add(4);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.scroll_y = app.scroll_y.saturating_add(2);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.scroll_y = app.scroll_y.saturating_sub(2);
                    }
                    KeyCode::Char('0') => {
                        app.scroll_x = 0;
                        app.scroll_y = 0;
                    }
                    KeyCode::Char('r') => {
                        app.reload_file();
                    }
                    _ => {}
                }
            }
        }
    }
}
