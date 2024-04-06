#![allow(unused)]
use std::env;

use workspaces::WorkSpace;

use crate::{
    tui::App,
    workspaces::{Monitor, State},
};

mod tui;
mod workspaces;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };
    tui::run(path)
}
