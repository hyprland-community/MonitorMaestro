use std::io::{stdout, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

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
    App::from_config(path)?.start_workspace(workspace)
}

pub fn get_state(path: &str) -> std::io::Result<()> {
    App::from_config(path)?.get_state()
}

pub fn run_tui(path: &str) -> std::io::Result<()> {
    let mut terminal = init()?;
    let _ = tui::App::from_config(path)?.run_tui(&mut terminal);
    restore()?;
    Ok(())
}
