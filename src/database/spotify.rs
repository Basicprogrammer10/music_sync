use derive_more::Deref;
use eyre::Result;
use rusqlite::Error;

use crate::misc::epoch;

use super::Db;

#[derive(Deref)]
pub struct SpotifyDb<'a>(pub(super) &'a Db);

impl<'a> SpotifyDb<'a> {
    pub fn get_auth(&self, id: &str) -> Result<Option<AuthCache>> {
        let query =
            self.read()
                .query_row("SELECT * FROM spotify_auth_cache WHERE id = ?", [id], |x| {
                    Ok(AuthCache {
                        id: x.get(0)?,
                        token: x.get(1)?,
                        refresh_token: x.get(2)?,
                        expires: x.get(3)?,
                    })
                });

        match query {
            Ok(e) => Ok(Some(e)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct AuthCache {
    id: String,
    token: String,
    refresh_token: String,
    expires: u64,
}

impl AuthCache {
    pub fn is_expired(&self) -> bool {
        epoch() >= self.expires
    }
}
