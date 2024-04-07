use clap::Parser;
use cli::Cli;

mod app;
mod cli;
mod tests;
mod workspaces;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let path = args.conf.as_deref().unwrap();
    match args.command {
        cli::Command::Tui => app::run_tui(path)?,
        cli::Command::GetState => app::get_state(path)?,
        cli::Command::StartWorkspace { name } => {
            app::start_workspace(path, &name)?;
        }
    }

    Ok(())
}
