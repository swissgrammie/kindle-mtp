use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use kindle_mtp::device::{FileEntry, Kindle};

struct App {
    kindle: Option<Kindle>,
    current_path: Vec<String>,
    entries: Vec<FileEntry>,
    list_state: ListState,
    status_message: String,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            kindle: None,
            current_path: vec![],
            entries: vec![],
            list_state: ListState::default(),
            status_message: "Press 'c' to connect to Kindle".to_string(),
            should_quit: false,
        }
    }

    fn connect(&mut self) {
        self.status_message = "Connecting to Kindle...".to_string();
        match Kindle::detect() {
            Ok(kindle) => {
                self.kindle = Some(kindle);
                self.status_message = "Connected! Loading files...".to_string();
                self.refresh_listing();
            }
            Err(e) => {
                self.status_message = format!("Connection failed: {}", e);
                self.kindle = None;
            }
        }
    }

    fn disconnect(&mut self) {
        self.kindle = None;
        self.entries.clear();
        self.current_path.clear();
        self.list_state.select(None);
        self.status_message = "Disconnected. Press 'c' to reconnect.".to_string();
    }

    fn current_path_string(&self) -> String {
        if self.current_path.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.current_path.join("/"))
        }
    }

    fn refresh_listing(&mut self) {
        if let Some(kindle) = &self.kindle {
            let path = self.current_path_string();
            match kindle.list_files(&path) {
                Ok(mut entries) => {
                    // Sort: folders first, then files, alphabetically
                    entries.sort_by(|a, b| {
                        match (a.is_folder, b.is_folder) {
                            (true, false) => std::cmp::Ordering::Less,
                            (false, true) => std::cmp::Ordering::Greater,
                            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                        }
                    });
                    self.entries = entries;
                    if !self.entries.is_empty() {
                        self.list_state.select(Some(0));
                    } else {
                        self.list_state.select(None);
                    }
                    self.status_message = format!("Path: {} ({} items)", path, self.entries.len());
                }
                Err(e) => {
                    self.status_message = format!("Error listing files: {}", e);
                }
            }
        }
    }

    fn enter_directory(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if let Some(entry) = self.entries.get(selected) {
                if entry.is_folder {
                    self.current_path.push(entry.name.clone());
                    self.refresh_listing();
                }
            }
        }
    }

    fn go_up(&mut self) {
        if !self.current_path.is_empty() {
            self.current_path.pop();
            self.refresh_listing();
        }
    }

    fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn select_previous(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') => {
                if self.kindle.is_none() {
                    self.connect();
                }
            }
            KeyCode::Char('d') => {
                if self.kindle.is_some() {
                    self.disconnect();
                }
            }
            KeyCode::Char('r') => {
                if self.kindle.is_some() {
                    self.refresh_listing();
                }
            }
            KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => self.enter_directory(),
            KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => self.go_up(),
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create app
    let mut app = App::new();

    // Main loop
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // File list
            Constraint::Length(3),  // Status
            Constraint::Length(3),  // Help
        ])
        .split(frame.area());

    // Title bar
    let connected = if app.kindle.is_some() { "CONNECTED" } else { "DISCONNECTED" };
    let title = format!(" Kindle File Browser [{}] ", connected);
    let title_block = Paragraph::new(title)
        .style(Style::default().fg(if app.kindle.is_some() { Color::Green } else { Color::Red }))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title_block, chunks[0]);

    // File list
    let path_display = app.current_path_string();
    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|entry| {
            let icon = if entry.is_folder { "📁" } else { "📄" };
            let size = if entry.is_folder {
                String::new()
            } else {
                format_size(entry.size)
            };
            let line = format!("{} {:<40} {:>10}", icon, entry.name, size);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .title(format!(" {} ", path_display))
            .borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // Status bar
    let status = Paragraph::new(format!(" {} ", app.status_message))
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    frame.render_widget(status, chunks[2]);

    // Help bar
    let help_text = if app.kindle.is_some() {
        " q:Quit | d:Disconnect | r:Refresh | ↑↓/jk:Navigate | Enter/→:Open | Backspace/←:Back "
    } else {
        " q:Quit | c:Connect "
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Help "));
    frame.render_widget(help, chunks[3]);
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}
