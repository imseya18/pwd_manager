use rusqlite::{Connection, Result};

pub trait Insertable {
  fn insert(&self, db: &Connection) -> Result<()>;
}
