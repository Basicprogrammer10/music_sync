use std::fmt::{Display, self};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Platforms {
    Spotify,
    AppleMusic,
}

trait Platform {}

impl Display for Platforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Platforms::Spotify => "spotify",
            Platforms::AppleMusic => "apple_music"
        })
    }
}
