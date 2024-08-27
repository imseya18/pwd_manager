use crate::entities::traits::Insertable;
use rusqlite::Connection;
use chrono::{Local, TimeZone, Utc};
use super::{Error, Result};

pub struct Account {
  db_id: Option<i64>,
  vault_id: i64,
  uid: String,
  sensitive_data: SensitiveData,
  created_at: i64,
  updated_at: i64
}

struct SensitiveData {
  label: Option<String>,
  name: String,
  account_name: String,
  password: String,
  url: Option<String>,
  note: Option<String>,
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
            sensitive_data: SensitiveData{
                label,
                name,
                account_name,
                password,
                url,
                note
            },
            created_at: Local::now().timestamp(),
            updated_at: Local::now().timestamp(),
        }
    }
}

// impl Insertable for Account {
//   fn insert(&self, db: &Connection) -> Result<()> {

//     db.execute("INSERT INTO account (id_vault, uid_account, sensitive_data, created_at, updated_at)
//                     VALUES  (?1, ?2, ?3, ?4, ?5)",
//       ( &self.vault_id ,&self.uid, &self.created_at, &self.updated_at))
//               .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//     Ok(())
//   }
// }
