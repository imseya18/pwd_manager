// use crate::{encryption, entities::traits::Insertable, Crypto};
// use crate::utils::convert_uid_from_db;
// use uuid::Uuid;
// use serde::{Deserialize, Serialize};
// use serde_json;
// use rusqlite::{Connection, params, Error as RusqliteError};
// use chrono::{Local, TimeZone, Utc};
// use super::{MyError, Result};

// #[derive(Debug)]
// pub struct Account {
//   db_id: Option<i64>,
//   vault_id: i64,
//   uid: Uuid,
//   sensitive_data: SensitiveData,
//   crypted_data: Option<Vec<u8>>,
//   created_at: i64,
//   updated_at: i64
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct SensitiveData {
//   label: Option<String>,
//   name: String,
//   account_name: String,
//   password: String,
//   url: Option<String>,
//   note: Option<String>,
// }

// impl Account {
//     pub fn new(
//         vault_id: i64,
//         name: impl Into<String>,
//         account_name: impl Into<String>,
//         password: impl Into<String>,
//         label: Option<String>,
//         url: Option<String>,
//         note: Option<String>,
//     ) -> Self {
//         Account {
//             db_id: None,
//             vault_id,
//             uid: Uuid::new_v4(),
//             sensitive_data: SensitiveData{
//                 label,
//                 name: name.into(),
//                 account_name: account_name.into(),
//                 password: password.into(),
//                 url,
//                 note
//             },
//             crypted_data: None,
//             created_at: Local::now().timestamp(),
//             updated_at: Local::now().timestamp(),
//         }
//     }

//     pub fn store_in_db(&mut self, encryption_key:&[u8; 32], vault_id: i64, db: &Connection) -> Result<()> {
//       let serialized_struct = serde_json::to_vec(&self.sensitive_data)?;
//       self.crypted_data = Some(Crypto::encrypt_for_storage(&serialized_struct, encryption_key)?);
//       self.insert(db)?;
//       Ok(())
//     }

//     pub fn get_all_account(vault_id:i64, encryption_key:&[u8; 32], db: &Connection) -> Result<Vec<Result<Account>>>{
//       let mut query = db.prepare("Select * FROM account WHERE id_vault = ?1")?;
//       let account_iter = query.query_map([vault_id], |row| {
//         let uid = convert_uid_from_db(row.get(2)?)?;
//         let encrypted_struct: Vec<u8> = row.get(3)?;
//         let decrypted_struct = Crypto::decrypt_from_storage(&encrypted_struct, encryption_key).map_err(|e| RusqliteError::UserFunctionError(Box::new(e)))?;
//         let sensitive_data: SensitiveData = serde_json::from_slice(&decrypted_struct).map_err(|e| RusqliteError::UserFunctionError(Box::new(e)))?;
//         Ok(Account{
//           db_id: row.get(0)?,
//           vault_id: row.get(1)?,
//           uid,
//           sensitive_data,
//           crypted_data: None,
//           created_at: row.get(4)?,
//           updated_at: row.get(5)?
//         })
//       })?;
//       let accounts: Vec<Result<Account>> = account_iter.map(|result| result.map_err(MyError::from)).collect();
//       if accounts.is_empty(){
//         return Err(MyError::Unknown("No accounts found for this user_id".to_string()));
//       }
//       Ok(accounts)
//     }
// }

// impl Insertable for Account {
//   fn insert(&self, db: &Connection) -> Result<()> {
//     db.execute("INSERT INTO account (id_vault, uid_account, sensitive_data, created_at, updated_at)
//                     VALUES  (?1, ?2, ?3, ?4, ?5)",
//       (&self.vault_id ,&self.uid.to_string(), &self.crypted_data ,&self.created_at, &self.updated_at))?;
//     Ok(())
//   }

//   fn delete(&self, db: &Connection) -> Result<()> {
//     let db_id = self.db_id.ok_or_else(|| MyError::Unknown("No Account_id found on this struct".to_string()))?;
//     db.execute("DELETE FROM account WHERE id_account = ?1", params![db_id])?;
//     Ok(())
//   }
// }
