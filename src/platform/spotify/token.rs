use std::sync::Arc;

use crate::{app::App, platform::Platform};

pub struct SpotifyToken {
    pub id: String,
    pub token: String,
    pub app: Arc<App>,
}

impl Platform for SpotifyToken {
    fn name(&self) -> &'static str {
        "Spotify"
    }

    fn sub_type(&self) -> &'static str {
        "token"
    }
}
