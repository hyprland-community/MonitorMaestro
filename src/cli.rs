use clap::{Parser, Subcommand};

#[derive(Debug, Default, Subcommand)]
pub enum Command {
    #[default]
    Tui,

    #[clap(name="state")]
    GetState,

    #[clap(name="workspace")]
    StartWorkspace{
        #[arg(index = 1)]
        name: String
    },
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// config file path
    #[arg(short, long, default_value = "./workspaces.json")]
    pub conf: Option<String>,

    /// Command to run
    #[command(subcommand)]
    pub command: Command,

    /// workspace at startup
    #[arg(short, long, default_value = None)]
    pub workspace: Option<String>,
}
