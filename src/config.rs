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
    use eyre::{bail, ContextCompat, Result};
    use hashbrown::HashMap;
    use serde::Deserialize;
    use toml::Value;
    use tracing::info;

    use crate::platform::PlatformConfig;

    #[derive(Debug)]
    pub struct PlatformConfigs {
        platforms: HashMap<String, PlatformConfig>,
    }

    impl PlatformConfigs {
        pub fn parse(str: &str) -> Result<Self> {
            let toml: Value = toml::from_str(str)?;
            let table = toml.as_table().context("Platform config is not a table")?;
            info!(
                "Found {} platforms: {}",
                table.len(),
                table.keys().cloned().collect::<Vec<_>>().join(", ")
            );

            let mut platforms = HashMap::new();

            for (key, value) in table {
                let platform = PlatformConfig::deserialize(value.to_owned())?;

                if platforms.contains_key(key) {
                    bail!("Duplicate platform identifier");
                }

                platforms.insert(key.to_ascii_lowercase(), platform);
            }

            Ok(Self { platforms })
        }

        pub fn supports(&self, id: &str) -> bool {
            self.platforms.contains_key(&id.to_ascii_lowercase())
        }
    }
}

/*
match platform {
                Platforms::Spotify => self.platforms.values().any(|x| {
                    matches!(
                        x,
                        PlatformConfig::SpotifyLogin { .. } | PlatformConfig::SpotifyToken { .. }
                    )
                }),
                Platforms::AppleMusic => false,
            }
*/
