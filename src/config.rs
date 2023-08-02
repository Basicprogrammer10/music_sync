use serde::Deserialize;

use crate::platform::Platforms;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub source: Source,
    pub drain: Drain,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub platform: Platforms,
    pub playlists: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Drain {
    pub platform: Platforms,
    pub create_playlist: bool,
}
