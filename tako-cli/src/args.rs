use std::borrow::Cow;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand, command, Args as ClapArgs};

#[derive(Parser)]
#[command(name = "Tk", about, long_about = None)]
pub(super) struct Args {
    #[command(subcommand)]
    pub(super) command: Command,

    #[arg(short = 'V', long)]
    pub(super) version: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Start(StartArgs),

    Echo {
        text: String,
    },
}

#[derive(ClapArgs, Debug)]
pub struct StartArgs {
    #[arg(long)]
    fast: bool,
}
