use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Platforms {
    Spotify,
    AppleMusic,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum PlatformConfig {
    #[serde(rename_all = "kebab-case")]
    SpotifyLogin {
        client_id: String,
        client_secret: String,
    },
    #[serde(rename_all = "kebab-case")]
    SpotifyToken { token: String },
}

trait Platform {}

impl Display for Platforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Platforms::Spotify => "spotify",
            Platforms::AppleMusic => "apple_music",
        })
    }
}
