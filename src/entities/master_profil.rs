use crate::entities::traits::Insertable;
use crate::anyhow;
use rusqlite::{Connection, params};
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use super::{Error, Result};

#[derive(Debug)]
pub enum MasterProfileError {
  RuSqliteError(rusqlite::Error),
  CrypteError(BcryptError),
  Other(String),
}

impl From<BcryptError> for MasterProfileError {
  fn from(err: BcryptError) -> Self{
    MasterProfileError::CrypteError(err)
  }
}

impl From<rusqlite::Error> for MasterProfileError {
  fn from(err: rusqlite::Error) -> Self {
      MasterProfileError::RuSqliteError(err)
  }
}

#[derive(Debug)]
pub struct MasterProfil {
  db_id: Option<i64>,
  pub uid: String,
  pub name: String,
  pub master_password: String,
  pub derivated_key: Option<[u8; 32]>
}


impl MasterProfil {

  pub fn create_store_new_profil(uid: impl Into<String>, name: impl Into<String>, master_password: impl Into<String>, db: &Connection) -> Result<Self> {
    Ok(Self::new(uid, name, master_password)
    .hash_password()?
    .self_insert(db)?)
  }

  pub fn new(uid: impl Into<String>, name: impl Into<String>, master_password: impl Into<String>) ->Self{
      MasterProfil {
        db_id: None,
        uid: uid.into(),
        name: name.into(),
        master_password: master_password.into(),
        derivated_key: None
      }
  }

  pub fn get_valide_existing_user(name: impl Into<String>, master_password: impl Into<String>, db: &Connection) ->Result<Self> {
      let user_from_db = Self::get_by_name(name.into(), db)?;
      Self::verify_password(&master_password.into(), &user_from_db.master_password )?;
      Ok(user_from_db)
  }

  pub fn hash_password(mut self) -> Result<Self>{
      self.master_password = hash(&self.master_password, DEFAULT_COST)?;
      Ok(self)
  }

  pub fn self_insert(self, db: &Connection) -> Result<Self> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.uid, &self.name, &self.master_password))?;
      Ok(self)
  }

  pub fn get_by_name(name: String ,db: &Connection) -> Result<Self> {
      Ok(db.query_row("SELECT * FROM master_profil WHERE name = ?1", params![name], |row| {
        Ok(MasterProfil {
                        db_id: row.get(0)?,
                        uid: row.get(1)?,
                        name: row.get(2)?,
                        master_password: row.get(3)?,
                        derivated_key: None
                    })
      })?)
  }

  pub fn verify_password(master_password: &str, hash: &str) ->Result<()>{
      if verify(master_password, hash)? {
        Ok(())
      }
      else {
        Err("Invalide Password".into())
      }
    }
}

impl Insertable for MasterProfil {
  fn insert(&self, db: &Connection) -> Result<()> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.uid, &self.name, &self.master_password))?;
      Ok(())
  }
}
