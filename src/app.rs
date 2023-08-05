use std::{
    fs::{self},
    sync::Arc,
};

use clap::Parser;
use eyre::{bail, Result};
use rusqlite::Connection;
use soon::Soon;
use tracing::info;

use crate::{
    args::Args,
    config::{config::Config, platform::PlatformConfigs},
    database::{Database, Db},
};

pub struct App {
    platform: Soon<PlatformConfigs>,
    config: Soon<Config>,
    database: Db,
}

impl App {
    pub fn new() -> Result<Arc<Self>> {
        let args = Args::parse();
        info!(
            "Loading config files: {:?}, {:?}",
            args.config, args.platform_config
        );

        let database = Db::new(Connection::open(args.database)?);
        database.init()?;

        let this = Arc::new(Self {
            platform: Soon::empty(),
            config: Soon::empty(),
            database,
        });

        let raw_platform = fs::read_to_string(&args.platform_config)?;
        let platform = PlatformConfigs::parse(&raw_platform, this.clone())?;
        this.platform.replace(platform);

        let raw_config = fs::read_to_string(args.config)?;
        let config = toml::from_str::<Config>(&raw_config)?;
        this.config.replace(config);

        for i in [&this.config.source.platform, &this.config.drain.platform]
            .iter()
            .filter(|&x| !this.platform.supports(x))
        {
            bail!(
                "Platform {} not configured in {:?}",
                i,
                args.platform_config
            );
        }

        Ok(this)
    }

    pub fn validate_tokens(&self) {}
}
