use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result, Error};
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};

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
  pub fn new_user(uid: impl Into<String>, name: impl Into<String>, master_password: impl Into<String>, db: &Connection) -> Result<Self, MasterProfileError> {
    Ok(MasterProfil {
      db_id: None,
      uid: uid.into(),
      name: name.into(),
      master_password: master_password.into(),
      derivated_key: None
    }
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

  pub fn hash_password(mut self) -> Result<Self, BcryptError>{
      self.master_password = hash(&self.master_password, DEFAULT_COST)?;
      Ok(self)
  }

  pub fn self_insert(self, db: &Connection) -> Result<Self, Error> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.uid, &self.name, &self.master_password))?;
      Ok(self)
  }
}

impl Insertable for MasterProfil {
  fn insert(&self, db: &Connection) -> Result<()> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.uid, &self.name, &self.master_password))?;
      Ok(())
  }
}
