use clap::Parser;
use cli::Cli;

mod app;
mod cli;
mod tests;
mod workspaces;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Command::Tui { mode } => match mode {
            cli::Mode::List { path } => app::run_list_tui(&path)?,
            cli::Mode::Interactive => app::run_interactive_tui()?,
        },
        cli::Command::GetState => app::get_state()?,
        cli::Command::StartWorkspace { path, name } => {
            app::start_workspace(&path, &name)?;
        }
        cli::Command::Monitors => app::get_monitors()?,
    }

    Ok(())
}
