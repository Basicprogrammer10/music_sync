use std::fs::{self};

use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;

use crate::{
    args::Args,
    config::config::Config,
    database::{Database, Db},
};

pub struct App {
    // platform: Platform,
    config: Config,
    database: Db,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();

        // let raw_platform = fs::read_to_string(&args.platform_config)?;
        // let platform = toml::from_str::<Platform>(&raw_platform)?;

        let raw_config = fs::read_to_string(args.config)?;
        let config = toml::from_str::<Config>(&raw_config)?;

        // for i in [config.source.platform, config.drain.platform]
        //     .iter()
        //     .filter(|&&x| !platform.supports(x))
        // {
        //     bail!("Platform {} not configured in {:?}", i, args.platform_config);
        // }

        let database = Db::new(Connection::open(args.database)?);
        database.init()?;

        Ok(Self {
            /*platform,*/ database,
            config,
        })
    }
}
