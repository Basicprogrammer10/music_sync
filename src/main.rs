use anyhow::Result;

mod app;
mod config;
mod platform;

fn main() -> Result<()> {
    let app = app::App::new()?;

    Ok(())
}
