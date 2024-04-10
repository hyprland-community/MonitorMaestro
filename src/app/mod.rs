use std::{
    ffi::OsStr,
    io::{stdout, Stdout},
    path::Path,
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::cli::Type;

use self::tui::App;

pub mod tui;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;


fn init() -> std::io::Result<Tui> {
    // execute!(stdout(), EnterAlternateScreen)?;
    let _ = stdout().execute(EnterAlternateScreen);
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore() -> std::io::Result<()> {
    let _ = stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}

pub fn start_workspace(path: &str, workspace: &str) -> std::io::Result<()> {
    let t = Path::new(path).extension().and_then(OsStr::to_str).unwrap_or_default();
    let t = if t == "json" {
        Type::Json
    } else if t == "toml" {
        Type::Toml
    } else {
        panic!("Wrong configuration file")
    };
    App::from_config(path, t)?.start_workspace(workspace)
}

pub fn get_state() -> std::io::Result<()> {
    App::get_state()
}

pub fn get_monitors() -> std::io::Result<()> {
    let _ = App::connected_monitors();

    Ok(())
}

pub fn run_list_tui(path: &str) -> std::io::Result<()> {
    let t = Path::new(path).extension().and_then(OsStr::to_str).unwrap_or_default();
    let t = if t == "json" {
        Type::Json
    } else if t == "toml" {
        Type::Toml
    } else {
        panic!("Wrong configuration file")
    };

    let mut terminal = init()?;
    let _ = tui::App::from_config(path, t)?.run_list_tui(&mut terminal);
    restore()?;
    Ok(())
}

pub fn run_interactive_tui() -> std::io::Result<()> {
    let mut terminal = init()?;
    tui::App::default().run_interactive_tui(&mut terminal)?;
    restore()?;
    Ok(())
}
