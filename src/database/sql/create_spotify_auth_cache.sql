CREATE TABLE IF NOT EXISTS spotify_auth_cache (
  id TEXT NOT NULL UNIQUE,
  token TEXT NOT NULL,
  refresh_token TEXT NOT NULL,
  expires INTEGER NOT NULL -- Unix Epoch (Secs)
)