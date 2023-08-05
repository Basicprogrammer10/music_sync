use anyhow::Result;

mod app;
mod config;
mod platform;
mod args;

fn main() -> Result<()> {
    let app = app::App::new()?;

    Ok(())
}
