tako_macro::pub_mod!(app confirm help input mgr notify);
tako_macro::flat_mod!(router ui);

use std::process::ExitCode;

// use clap::Parser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::App::serve().await
}
