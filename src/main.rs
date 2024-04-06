use clap::Parser;
use cli::Cli;

mod cli;
mod tests;
mod tui;
mod workspaces;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if args.tui == true {
        let path = args.conf.as_deref();
        tui::run(path)?;
    }

    Ok(())
}
