use std::fmt::{self, Display};

use eyre::Result;
use serde::Deserialize;

pub mod spotify;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Platforms {
    Spotify,
    AppleMusic,
}

pub trait Platform {
    // == Info ==
    fn name(&self) -> &'static str;
    fn sub_type(&self) -> &'static str {
        ""
    }

    // == Actions ==
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Display for Platforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Platforms::Spotify => "spotify",
            Platforms::AppleMusic => "apple_music",
        })
    }
}
