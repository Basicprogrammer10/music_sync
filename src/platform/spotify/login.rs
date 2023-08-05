use std::sync::Arc;

use crate::{app::App, platform::Platform};

pub struct SpotifyLogin {
    pub client_id: String,
    pub client_secret: String,
    pub app: Arc<App>,
}

impl Platform for SpotifyLogin {
    fn name(&self) -> &'static str {
        "Spotify"
    }

    fn sub_type(&self) -> &'static str {
        "oauth-login"
    }
}
