use rusqlite::Connection;
use super::{Error, Result};

pub trait Insertable {
    fn insert(&self, db: &Connection) -> Result<()>;
}
