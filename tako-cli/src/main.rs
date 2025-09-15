use std::process::ExitCode;

use anyhow::Result;
use tako_core::App;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;
    Ok(())
}
