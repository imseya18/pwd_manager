use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};

#[derive(Debug)]
pub struct MasterProfil {
  db_id: Option<i64>,
  pub uid: String,
  pub name: String,
  pub master_password: String,
}


impl MasterProfil {
  pub fn new(uid: String, name: String, master_password: String) ->Self{
      MasterProfil {
        db_id: None,
        uid,
        name,
        master_password,
      }
  }

  pub fn hash_password(mut self) -> Result<Self, BcryptError>{
      self.master_password = hash(&self.master_password, DEFAULT_COST)?;
      Ok(self)
  }

  pub fn self_insert(self, db: &Connection) -> Result<Self> {
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
