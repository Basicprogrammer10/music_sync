CREATE TABLE IF NOT EXISTS spofity_auth_cache (
  id TEXT NOT NULL,
  token TEXT NOT NULL,
  refresh_token TEXT NOT NULL,
  expires_in INTEGER NOT NULL -- Unix Epoch (Secs)
)