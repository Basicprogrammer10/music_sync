use eyre::Result;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod app;
mod args;
mod config;
mod database;
mod misc;
mod platform;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app::App::new()?;
    app.validate_tokens()?;

    app.database.cleanup()?;
    Ok(())
}
