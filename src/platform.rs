use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Platforms {
    Spotify,
    AppleMusic,
}

trait Platform {}
