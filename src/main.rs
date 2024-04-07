use clap::Parser;
use cli::Cli;

mod app;
mod cli;
mod tests;
mod workspaces;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let path = args.conf.as_deref().unwrap();
    if args.workspace.is_some() {
        app::start_workspace(path, args.workspace.as_deref().unwrap())?;
    } else if args.tui == true {
        app::run_tui(path)?;
    }

    Ok(())
}
