use anyhow::Result;
use tako_widgets::UiState;

pub fn run(running: &mut bool) -> Result<()> {
    *running = false;
    Ok(())
}
