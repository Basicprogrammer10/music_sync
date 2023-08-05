use std::fs::{self};

use anyhow::{bail, Result};
use clap::Parser;

use crate::{
    args::Args,
    config::{config::Config, platform::Platform},
};

pub struct App {
    platform: Platform,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();

        let raw_platform = fs::read_to_string(&args.platform_config)?;
        let platform = toml::from_str::<Platform>(&raw_platform)?;

        let raw_config = fs::read_to_string(args.config)?;
        let config = toml::from_str::<Config>(&raw_config)?;

        for i in [config.source.platform, config.drain.platform]
            .iter()
            .filter(|&&x| !platform.supports(x))
        {
            bail!("Platform {} not configured in {:?}", i, args.platform_config);
        }

        Ok(Self { platform, config })
    }
}
