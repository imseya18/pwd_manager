use crate::entities::traits::Insertable;
use rusqlite::{Connection, params};
use chrono::{Local, TimeZone, Utc};
use super::{Error, Result};

#[derive(Debug)]
pub struct Vault {
  db_id: Option<i64>,
  uid: String,
  user_id: i64,
  name: String,
  created_at: i64,
  updated_at: i64
}

impl  Vault {
  pub fn new(uid: impl Into<String>, user_id: i64, name: impl Into<String>) -> Self {
    Vault{
      db_id: None,
      uid: uid.into(),
      user_id,
      name: name.into(),
      created_at: Local::now().timestamp(),
      updated_at: Local::now().timestamp()
      }
  }

  pub fn create_store_in_db(uid: impl Into<String>, user_id:i64 ,name: impl Into<String>, db: &Connection) -> Result<Self> {
      let vault = Self::new(uid, user_id, name);
      vault.insert(db)?;
      Ok(vault)
  }

  pub fn get_by_user_id(user_id:i64, db: &Connection) -> Result<Vec<Vault>> {
    let mut query = db.prepare("Select * FROM vault WHERE id_profil = ?1")?;
    let vaults_itter = query.query_map([user_id], |row| {
      Ok(Vault {
        db_id: row.get(0)?,
        user_id: row.get(1)?,
        uid: row.get(2)?,
        name: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?
        })
    })?;

    /*Store only valide data ignore errors*/
    let vaults: Vec<Vault> = vaults_itter
        .filter_map(|result| result.ok())
        .collect();

    Ok(vaults)
  }
}

impl Insertable for Vault {
  fn insert(&self, db: &rusqlite::Connection) -> Result<()> {
    db.execute("INSERT INTO vault (uid_vault, id_profil, name, created_at, updated_at) VALUES  (?1, ?2, ?3, ?4, ?5)",
      (&self.uid, &self.user_id,  &self.name, &self.created_at, &self.updated_at))?;
    Ok(())
  }
}
