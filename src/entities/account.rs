use crate::entities::traits::Insertable;
use rusqlite::Connection;
use chrono::{Local, TimeZone, Utc};
use super::{Error, Result};

pub struct Account {
  db_id: Option<i64>,
  vault_id: i64,
  uid: String,
  label: Option<String>,
  name: String,
  account_name: String,
  password: String,
  url: Option<String>,
  note: Option<String>,
  created_at: i64,
  updated_at: i64
}

impl Account {
    pub fn new(
        vault_id: i64,
        uid: String,
        name: String,
        account_name: String,
        password: String,
        label: Option<String>,
        url: Option<String>,
        note: Option<String>,
    ) -> Self {
        Account {
            db_id: None,
            vault_id,
            uid,
            label,
            name,
            account_name,
            password,
            url,
            note,
            created_at: Local::now().timestamp(),
            updated_at: Local::now().timestamp(),
        }
    }
}

impl Insertable for Account {
  fn insert(&self, db: &Connection) -> Result<()> {
    db.execute("INSERT INTO account (id_vault, uid_account, name, label, account_name, password, url, note, created_at, updated_at)
                    VALUES  (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
      ( &self.vault_id ,&self.uid, &self.name,  &self.label, &self.account_name, &self.password, &self.url, &self.note, &self.created_at, &self.updated_at))
              .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(())
  }
}
