use crate::entities::traits::Insertable;
use crate::utils::convert_uid_from_db;
use rusqlite::{Connection, params, Error as RusqliteError};
use chrono::{Local, TimeZone, Utc};
use uuid::Uuid;
use super::Result;

#[derive(Debug)]
pub struct Vault {
  db_id: Option<i64>,
  uid: Uuid,
  user_id: i64,
  name: String,
  created_at: i64,
  updated_at: i64
}

impl  Vault {
  pub fn new(user_id: i64, name: impl Into<String>) -> Self {
    Vault{
      db_id: None,
      uid: Uuid::new_v4(),
      user_id,
      name: name.into(),
      created_at: Local::now().timestamp(),
      updated_at: Local::now().timestamp()
      }
  }

  pub fn create_store_in_db(user_id:i64 ,name: impl Into<String>, db: &Connection) -> Result<Self> {
      let vault = Self::new(user_id, name);
      vault.insert(db)?;
      Ok(vault)
  }

  pub fn get_by_user_id(user_id:i64, db: &Connection) -> Result<Vec<Result<Vault>>> {
    let mut query = db.prepare("Select * FROM vault WHERE id_profil = ?1")?;
    let vaults_itter = query.query_map([user_id], |row| {
      let uid = convert_uid_from_db(row.get(2)?)?;
      Ok(Vault {
        db_id: row.get(0)?,
        user_id: row.get(1)?,
        uid,
        name: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?
        })
    })?;

    /*contain Vec of result*/
    let vaults:Vec<Result<Vault>> = vaults_itter.map(|result| result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)).collect();
    if vaults.is_empty(){
      return Err(Box::from("No vault found for this user_id"));
    }
    Ok(vaults)
  }
}

impl Insertable for Vault {
  fn insert(&self, db: &rusqlite::Connection) -> Result<()> {
    let encrypted_struct =
    db.execute("INSERT INTO vault (uid_vault, id_profil, name, created_at, updated_at) VALUES  (?1, ?2, ?3, ?4, ?5)",
      (&self.uid.to_string(), &self.user_id,  &self.name, &self.created_at, &self.updated_at))?;
    Ok(())
  }

  fn delete(&self, db: &Connection) -> Result<()> {
    let db_id = self.db_id.ok_or_else(|| MyError::Unknown("No Account_id found on this struct".to_string()))?;
    db.execute("DELETE FROM vault WHERE id_vault = ?1", params![db_id])?;
    Ok(())
  }
}
