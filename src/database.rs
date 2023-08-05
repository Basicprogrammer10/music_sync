use anyhow::Result;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use rusqlite::Connection;

pub struct Db {
    inner: RwLock<Option<Connection>>,
}

pub trait Database {
    // == Base ==
    fn init(&self) -> Result<()>;
    fn cleanup(&self) -> Result<()>;
}

impl Db {
    pub fn new(connection: Connection) -> Self {
        Self {
            inner: RwLock::new(Some(connection)),
        }
    }

    fn take(&self) -> Connection {
        let val = self.inner.write().take();
        val.expect("No value to take")
    }

    fn read(&self) -> MappedRwLockReadGuard<'_, Connection> {
        RwLockReadGuard::map(self.inner.read(), |x| x.as_ref().expect("No value to take"))
    }

    fn write(&self) -> MappedRwLockWriteGuard<'_, Connection> {
        RwLockWriteGuard::map(self.inner.write(), |x| {
            x.as_mut().expect("No value to take")
        })
    }
}

impl Database for Db {
    fn init(&self) -> Result<()> {
        let mut this = self.write();
        this.pragma_update(None, "journal_mode", "WAL")?;
        this.pragma_update(None, "synchronous", "NORMAL")?;

        let trans = this.transaction()?;
        for i in [include_str!("./sql/create_spotify_auth_cache.sql")] {
            trans.execute(i, [])?;
        }
        trans.commit()?;
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        let this = self.take();
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        drop(this);
        Ok(())
    }
}