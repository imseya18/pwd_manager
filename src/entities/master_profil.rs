use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result};

pub struct MasterProfil {
  pub uid: String,
  pub name: String,
  pub master_password: String,
}

impl MasterProfil {
  pub fn new(uid: String, name: String, master_password: String) ->Self{
      MasterProfil {
        uid,
        name,
        master_password,
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
