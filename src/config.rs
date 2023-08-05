pub mod config {
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
}

pub mod platform {
    use serde::Deserialize;

    use crate::platform::Platforms;

    #[derive(Deserialize, Debug)]
    pub struct Platform {
        pub spotify: Option<Spotify>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="kebab-case")]
    pub struct Spotify {
        pub client_id: String,
        pub client_secret: String,
    }
    
    impl Platform {
        pub fn supports(&self, platform: Platforms) -> bool {
            match platform {
                Platforms::Spotify => self.spotify.is_some(),
                Platforms::AppleMusic => false
            }
        }
    }
}
