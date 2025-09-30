tako_macro::pub_mod!(app confirm help input mgr notify);
tako_macro::flat_mod!(router);

// use clap::Parser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tako_shared::event::Event::init();
    app::App::serve().await
}
