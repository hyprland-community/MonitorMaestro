use std::{
    io::{stdout, Stdout},
    process::Command,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};

use crate::workspaces::WorkSpace;
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> std::io::Result<Tui> {
    // execute!(stdout(), EnterAlternateScreen)?;
    let _ = stdout().execute(EnterAlternateScreen);
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> std::io::Result<()> {
    let _ = stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}

pub fn run(path: Option<&str>) -> std::io::Result<()> {
    let mut terminal = init()?;
    let _ = App::from_config(path)?.run(&mut terminal);
    restore()?;
    Ok(())
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    workspaces: Vec<WorkSpace>,

    #[serde(skip)]
    index: usize,
    #[serde(skip)]
    exit: bool,
}

impl App {
    pub fn new(workspaces: Vec<WorkSpace>) -> Self {
        Self {
            index: 0,
            workspaces,
            exit: false,
        }
    }

    pub fn from_config(path: Option<&str>) -> std::io::Result<Self> {
        let path = match path {
            Some(p) => p,
            None => "./workspaces.json",
        };

        let data = std::fs::read_to_string(path)?;
        let app: App = serde_json::from_str(&data)?;

        Ok(app)
    }

    pub fn run(&mut self, terminal: &mut Tui) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, f: &mut Frame) {
        let title = Title::from("WorkSpace selector");
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Center);

        let mut list = Vec::<ListItem>::new();

        for (i, ws) in self.workspaces.iter().enumerate() {
            let style = if i == self.index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            list.push(ListItem::new(
                Line::from(Span::from(format!("{}", ws.workspace_name)))
                    .alignment(Alignment::Center)
                    .style(style),
            ));
        }

        let list = List::new(list).block(block).highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        );
        f.render_widget(list, f.size());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> std::io::Result<()> {
        match key_event.code {
            KeyCode::Char('j') => {
                if self.index < self.workspaces.len() - 1 {
                    self.index += 1;
                }
            }
            KeyCode::Char('k') => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            KeyCode::Enter => self.execute_selected(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }

        Ok(())
    }

    fn execute_selected(&mut self) {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(self.workspaces[self.index].command())
            .output();
    }
}
