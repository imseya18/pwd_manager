use rusqlite::Connection;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use super::Result;

pub trait Insertable {
    fn insert(&self, db: &Pool<SqliteConnectionManager>) -> Result<()>;
    fn delete(&self, db: &Connection) -> Result<()>;
}
