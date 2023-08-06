use std::{sync::Arc, thread};

use afire::{
    internal::encoding::base64, Content, HeaderType, Method, Middleware, Request, Response, Server,
    Status,
};
use crossbeam::channel::Sender;
use eyre::Result;
use serde::Deserialize;
use tracing::{error, info, warn};

use crate::{app::App, database::spotify::AuthCache, misc::rand_str, platform::Platform};

const SCOPES: &[&str] = &[
    "playlist-read-private",
    "playlist-modify-private",
    "user-read-private",
];

pub struct SpotifyLogin {
    pub id: String,
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

    fn validate(&self) -> Result<()> {
        // - Check that database has an entry for id - if not, login
        // - Check that token is not expired - if it is refresh it - if refresh fails - login

        let spotify_db = self.app.database.spotify();
        let auth = match spotify_db.get_auth(&self.id)? {
            Some(e) => e,
            None => {
                warn!("No token found in database for `{}`", self.id);
                login(self)?;
                return Ok(());
            }
        };

        if auth.is_expired() {
            // REAUTH
        }

        Ok(())
    }
}

fn login(login: &SpotifyLogin) -> Result<AuthCache> {
    struct State {
        client_id: String,
        client_secret: String,
        state: String,
        redirect_uri: String,
        oauth_port: u16,
        tx: Sender<ResponseBody>,
    }

    #[derive(Debug, Deserialize)]
    struct ResponseBody {
        access_token: String,
        refresh_token: String,
        expires_in: u64,
    }

    let (tx, rx) = crossbeam::channel::bounded(1);
    let state = State {
        client_id: login.client_id.to_owned(),
        client_secret: login.client_secret.to_owned(),
        state: rand_str(10),
        redirect_uri: format!("http://localhost:{}/callback", login.app.args.oauth_port),
        oauth_port: login.app.args.oauth_port,
        tx,
    };

    thread::spawn(move || {
        let mut server = Server::<State>::new([127, 0, 0, 1], state.oauth_port).state(state);

        server.stateful_route(Method::GET, "/login", move |app, _req| {
            let mut location = url_builder::URLBuilder::new();
            location.set_protocol("https");
            location.set_host("accounts.spotify.com/authorize");
            location.add_param("client_id", &app.client_id);
            location.add_param("scope", &SCOPES.join(" "));
            location.add_param("redirect_uri", &app.redirect_uri);
            location.add_param("state", &app.state);
            location.add_param("response_type", "code");
            let location = location.build();

            Response::new()
                .text(format!(
                    r#"Redirecting to <a href="{location}">{location}</a>"#
                ))
                .status(Status::TemporaryRedirect)
                .header(HeaderType::Location, location)
                .content(Content::HTML)
        });

        server.stateful_route(Method::GET, "/callback", move |app, req| {
            let local_state = match req.query.get("state") {
                Some(e) => e,
                None => return Response::new().status(500).text("No state parameter"),
            };

            if local_state != app.state {
                return Response::new()
                    .status(500)
                    .text("Incorrect state parameter");
            }

            if let Some(i) = req.query.get("error") {
                return Response::new()
                    .status(500)
                    .text(format!("OAUTH Error: {i}"));
            }

            let code = match req.query.get("code") {
                Some(e) => e,
                None => return Response::new().status(500).text("No code parameter"),
            };

            let auth = format!("{}:{}", app.client_id, app.client_secret);
            let res: ResponseBody = match ureq::post("https://accounts.spotify.com/api/token")
                .set(
                    "Authorization",
                    &format!("Basic {}", &base64::encode(auth.as_bytes())),
                )
                .send_form(&[
                    ("grant_type", "authorization_code"),
                    ("code", code),
                    ("redirect_uri", &app.redirect_uri),
                ]) {
                Ok(e) => e.into_json().unwrap(),
                Err(e) => {
                    return Response::new()
                        .status(500)
                        .text(format!("Error getting auth token: {e}"))
                }
            };
            app.tx.send(res).unwrap();

            Response::new().text("Ok! You can now close this tab.")

            // TODO: kill self somehow?
        });

        info!(
            "Go to http://localhost:{}/login to connect with spotify",
            server.app().oauth_port
        );
        server.start().unwrap();
    });

    let response = rx.recv().unwrap();
    dbg!(response);

    todo!()
}
