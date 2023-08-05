use std::fmt::{self, Display};

use serde::Deserialize;

pub mod spotify;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Platforms {
    Spotify,
    AppleMusic,
}

pub trait Platform {
    fn name(&self) -> &'static str;
    fn sub_type(&self) -> &'static str {
        ""
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
