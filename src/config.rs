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
        pub platform: String,
        pub playlists: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Drain {
        pub platform: String,
        pub create_playlist: bool,
    }
}

pub mod platform {
    use serde::Deserialize;

    use crate::platform::{Platforms, PlatformConfig};

    #[derive(Debug)]
    pub struct PlatformConfigs {
        platforms: Vec<PlatformConfig>
    }


    pub struct Spotify {
        pub client_id: String,
        pub client_secret: String,
    }
    
    // impl Platform {
    //     pub fn supports(&self, platform: Platforms) -> bool {
    //         match platform {
    //             Platforms::Spotify => self.spotify.is_some(),
    //             Platforms::AppleMusic => false
    //         }
    //     }
    // }
}
