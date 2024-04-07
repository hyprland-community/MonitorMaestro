use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// config file path
    #[arg(short, long, default_value = "./workspaces.json")]
    pub conf: Option<String>,

    /// workspace at startup
    #[arg(short, long, default_value = None)]
    pub workspace: Option<String>,

    /// start in tui mode
    #[arg(short, long, default_value = "false")]
    pub tui: bool,
}
