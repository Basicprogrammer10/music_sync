use super::Platform;

pub struct SpotifyLogin {
    pub client_id: String,
    pub client_secret: String,
}

pub struct SpotifyToken {
   pub token: String,
}

impl Platform for SpotifyLogin {
    fn name(&self) -> &'static str {
        "Spotify"
    }
    
    fn sub_type(&self) -> &'static str {
        "oauth-login"
    }
}

impl Platform for SpotifyToken {
    fn name(&self) -> &'static str {
        "Spotify"
    }

    fn sub_type(&self) -> &'static str {
        "token"
    }
}
