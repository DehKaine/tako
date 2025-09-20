tako_macro::pub_mod!(package);
tako_macro::flat_mod!(args);

use std::process::ExitCode;

use clap::Parser;
use anyhow::Result;
use tako_core::App;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;
    Ok(())
}
