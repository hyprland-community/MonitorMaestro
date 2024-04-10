use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Default, Subcommand)]
pub enum Mode {
    #[clap(name = "list")]
    List {
        #[arg(short, long)]
        conf: String,
    },

    #[default]
    #[clap(name = "interactive")]
    Interactive,
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
        /// path to the config file
        #[arg(short, long)]
        conf: String,

        /// name of the monitor layout to start
        #[arg(short, long)]
        name: String,
    },

    /// get attached monitors
    #[clap(name = "monitors")]
    Monitors,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Type {
    Json,
    Toml,
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// Command to run
    #[command(subcommand)]
    pub command: Command,
}
