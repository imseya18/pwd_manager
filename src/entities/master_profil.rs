use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result};

pub struct MasterProfil {
  pub id: i64,
  pub name: String,
  pub master_password: String,
}

impl MasterProfil {
  pub fn new(id: i64, name: String, master_password: String) ->Self{
      MasterProfil {
        id,
        name,
        master_password,
      }
  }
}

impl Insertable for MasterProfil {
  fn insert(&self, db: &Connection) -> Result<()> {
      db.execute("INSERT INTO master_profil (id_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.id, &self.name, &self.master_password))?;
      Ok(())
  }
}
