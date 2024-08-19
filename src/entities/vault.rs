use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result};
use chrono::{Local, TimeZone, Utc};

#[derive(Debug)]
pub struct Vault {
  id: String,
  user_id: String,
  name: String,
  created_at: i64,
  updated_at: i64
}

impl  Vault {
  pub fn new(id: String, user_id: String, name: String) -> Self {
    Vault{
      id,
      user_id,
      name,
      created_at: Local::now().timestamp(),
      updated_at: Local::now().timestamp()
      }
  }
}

impl Insertable for Vault {
  fn insert(&self, db: &rusqlite::Connection) -> Result<()> {
    db.execute("INSERT INTO vault (id_vault, id_profil, name, created_at, updated_at) VALUES  (?1, ?2, ?3, 4?, 5?)",
      (&self.id, &self.user_id,  &self.name, &self.created_at, &self.updated_at))?;
    Ok(())
  }
}
