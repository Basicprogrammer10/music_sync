pub mod config {
    use serde::Deserialize;

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
    use std::{fmt::Debug, sync::Arc};

    use eyre::{bail, ContextCompat, Result};
    use hashbrown::HashMap;
    use serde::Deserialize;
    use toml::Value;
    use tracing::info;

    use crate::{
        app::App,
        platform::{
            spotify::{login::SpotifyLogin, token::SpotifyToken},
            Platform,
        },
    };
    pub struct PlatformConfigs {
        pub platforms: HashMap<String, Box<dyn Platform>>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "type", rename_all = "kebab-case")]
    enum PlatformConfig {
        #[serde(rename_all = "kebab-case")]
        SpotifyLogin {
            client_id: String,
            client_secret: String,
        },
        #[serde(rename_all = "kebab-case")]
        SpotifyToken { token: String },
    }

    impl PlatformConfigs {
        pub fn parse(str: &str, app: Arc<App>) -> Result<Self> {
            let toml: Value = toml::from_str(str)?;
            let table = toml.as_table().context("Platform config is not a table")?;
            info!(
                "Found {} platforms: {}",
                table.len(),
                table.keys().cloned().collect::<Vec<_>>().join(", ")
            );

            let mut platforms = HashMap::new();

            for (key, value) in table {
                let raw_platform = PlatformConfig::deserialize(value.to_owned())?;

                if platforms.contains_key(key) {
                    bail!("Duplicate platform identifier");
                }

                platforms.insert(
                    key.to_ascii_lowercase(),
                    raw_platform.into_object(key.to_owned(), app.clone()),
                );
            }

            Ok(Self { platforms })
        }

        pub fn supports(&self, id: &str) -> bool {
            self.platforms.contains_key(&id.to_ascii_lowercase())
        }
    }

    impl PlatformConfig {
        fn into_object(self, id: String, app: Arc<App>) -> Box<dyn Platform> {
            match self {
                PlatformConfig::SpotifyLogin {
                    client_id,
                    client_secret,
                } => Box::new(SpotifyLogin {
                    client_id,
                    client_secret,
                    id: id.to_owned(),
                    app: app.clone(),
                }),
                PlatformConfig::SpotifyToken { token } => Box::new(SpotifyToken {
                    token,
                    id: id.to_owned(),
                    app: app.clone(),
                }),
            }
        }
    }

    impl Debug for PlatformConfigs {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PlatformConfigs")
                .field(
                    "platforms",
                    &self
                        .platforms
                        .iter()
                        .map(|x| {
                            (
                                x.0,
                                format!(
                                    "{}{}{}",
                                    x.1.name(),
                                    if x.1.sub_type().is_empty() { "" } else { " - " },
                                    x.1.sub_type()
                                ),
                            )
                        })
                        .collect::<HashMap<_, _>>(),
                )
                .finish()
        }
    }
}
