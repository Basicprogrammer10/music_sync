use anyhow::Result;

mod app;
mod args;
mod config;
mod database;
mod platform;

fn main() -> Result<()> {
    let app = app::App::new()?;

    Ok(())
}
