use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Default, Subcommand)]
pub enum Mode {
    #[default]
    #[clap(name = "list")]
    List,

    #[clap(name = "interactive")]
    Interactive
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// start in tui mode
    Tui {
        #[command(subcommand)]
        mode: Mode,
    },

    /// get current monitor layout 
    #[clap(name = "state")]
    GetState,

    /// start specified monitor layout
    #[clap(name = "workspace")]
    StartWorkspace {
        #[arg(index = 1)]
        name: String,
    },

    /// get attached monitors
    #[clap(name = "monitors")]
    Monitors
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
