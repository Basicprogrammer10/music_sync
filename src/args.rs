use std::path::PathBuf;

use clap::clap_derive::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    pub config: PathBuf,
    pub platform_config: PathBuf,
    #[arg(default_value = "data.db")]
    pub database: PathBuf,
    #[arg(default_value_t = 8231)]
    pub oauth_port: u16,
}
