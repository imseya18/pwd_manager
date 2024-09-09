use rusqlite::Connection;
use super::Result;

pub trait Insertable {
    fn insert(&self, db: &Connection) -> Result<()>;
    fn delete(&self, db: &Connection) -> Result<()>;
}
