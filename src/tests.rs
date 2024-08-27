#[cfg(test)]

use crate::Crypto;
use crate::MasterProfil;
use crate::{entities::traits::Insertable, Vault};
use bcrypt::{hash, verify, DEFAULT_COST};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

use rusqlite::Connection;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serde_json;
use aes::{Aes256, BlockEncrypt, NewBlockCipher};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;



fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute_batch(
    "PRAGMA foreign_keys = ON;

      CREATE TABLE if not exists master_profil (
          id_profil INTEGER PRIMARY KEY AUTOINCREMENT,
          uid_profil TEXT not null unique,
          name TEXT not null unique,
          master_password TEXT not null);

      CREATE TABLE if not exists vault (
          id_vault INTEGER PRIMARY KEY AUTOINCREMENT,
          id_profil INTEGER REFERENCES master_profil (id_profil) not null,
          uid_vault TEXT not null unique,
          name TEXT not null,
          created_at TIMESTAMP not null,
          updated_at TIMESTAMP);

      CREATE TABLE if not exists account (
          id_account INTEGER PRIMARY KEY AUTOINCREMENT,
          id_vault INTEGER REFERENCES vault (id_vault) not null,
          uid_account TEXT not null unique,
          name TEXT not null,
          label TEXT,
          account_name TEXT not null,
          password TEXT not null,
          url TEXT,
          note TEXT,
          created_at TIMESTAMP not null,
          updated_at TIMESTAMP)")?;
    Ok(conn)
}

#[test]
fn insert_duplicate_profile_fails() {
    let conn = setup_test_db().expect("Failed to set up test database");

    // Création du premier profil
    let new_profil = MasterProfil::new(
        "test_password".to_string(),
        "test_hash".to_string()
    );

    // Insère le premier profil
    assert!(new_profil.insert(&conn).is_ok(), "Failed to insert first master profil");

    // Tentative d'insertion du même profil une deuxième fois
    let result = new_profil.insert(&conn);

    assert!(result.is_err(), "Duplicate profile insertion did not fail as expected");
}

#[test]
fn insert_profil(){
let conn = setup_test_db().expect("Failed to set up test database");
let new_profil = MasterProfil::new(
    "test_password".to_string(),
    "test_hash".to_string(),
);
assert!(new_profil.insert(&conn).is_ok(), "Failed to insert first master profil");
}

#[test]
fn connect_to_db(){
  let conn = setup_test_db();
  assert!(conn.is_ok())
}

#[test]
fn verify_salt(){
    let password = "this is a test password";
  let hashed_password = hash(password, DEFAULT_COST).unwrap();
  let hashed_password2 = hash(password, DEFAULT_COST).unwrap();
  assert_ne!(hashed_password, hashed_password2)
}

#[test]
fn get_master_profil_from_db_ok(){
  let db = setup_test_db().expect("failed to connect to db");
  let new_profil = MasterProfil::create_store_in_db( "JGLP", "c'est ok", &db).expect("failed to create profil");
  let get_profil = MasterProfil::get_by_name("JGLP", &db).expect("probleme");

  assert_eq!(new_profil.name, get_profil.name);
  assert_eq!(new_profil.uid, get_profil.uid);
  assert_eq!(new_profil.master_password, get_profil.master_password);
}

#[test]
fn get_master_profil_from_db_error(){
  let db = setup_test_db().expect("failed to connect to db");
  let get_profil = MasterProfil::get_by_name("JGLP", &db);
  assert_eq!(get_profil.is_ok(), false);
}

#[test]
fn good_user_good_password() -> Result<()>{
  let db = setup_test_db().expect("failed to connect to db");
  let insert_profil = MasterProfil::create_store_in_db(
    "JGLP2", "1234",
    &db)?;
  let get_profil = MasterProfil::get_valide_existing_user("JGLP2", "1234", &db);
  assert!(get_profil.is_ok(), "Expected no error due to good password");
  Ok(())
}

#[test]
fn good_user_wrong_password() -> Result<()>{
  let db = setup_test_db().expect("failed to connect to db");
  let insert_profil = MasterProfil::create_store_in_db(
    "JGLP2", "1234",
    &db)?;
  let get_profil = MasterProfil::get_valide_existing_user("JGLP2", "dfsdgdg", &db);
  assert!(get_profil.is_err(), "Expected an error due to wrong password");
  Ok(())
}

#[test]
fn insert_new_vault() -> Result<()>{
    let db = setup_test_db().expect("failed to connect to db");
    let main_profil = MasterProfil::create_store_in_db(
      "JGLP2", "1234",
      &db)?;
    let profil_from_db = MasterProfil::get_valide_existing_user("JGLP2", "1234", &db)?;
    let new_vault = Vault::new(profil_from_db.db_id.ok_or("User_id is None")?, "this is a new vault")
      .insert(&db);
    assert!(new_vault.is_ok(), "Expected no Error all value are good");
    Ok(())
}

#[test]
fn insert_new_vault_wrong_user_id() -> Result<()>{
    let db = setup_test_db().expect("failed to connect to db");
    let new_vault = Vault::new( 457457, "this is a new vault")
      .insert(&db);
    assert!(new_vault.is_err(), "Expected Error user_id not exist");
    Ok(())
}

fn get_new_vault() -> Result<()>{
      let db = setup_test_db().expect("failed to connect to db");
      let main_profil = MasterProfil::create_store_in_db(
        "JGLP2", "1234",
        &db)?;
      let profil_from_db = MasterProfil::get_valide_existing_user("JGLP2", "1234", &db)?;
      let new_vault = Vault::new(profil_from_db.db_id.ok_or("User_id is None")?, "this is a new vault")
        .insert(&db);
      let new_vault = Vault::new(profil_from_db.db_id.ok_or("User_id is None")?, "this is a new vault2")
        .insert(&db);

      let vaults_result = Vault::get_by_user_id(profil_from_db.db_id.ok_or(err), &db)
      Ok(())
}
/* ========== CRYPTO ========== */

#[test]
fn salt_randmoness() {
    let mut salts = HashSet::new();

    for _ in 0..10 {
        let salt = Crypto::generate_rnd_salt();
        salts.insert(salt);
    }

    assert_eq!(salts.len(), 10);
}

#[test]
fn key_from_password() {

    let mut  passwords: Vec<(&str, [u8; 16], [u8; 32])> = Vec::new();

    passwords.push(("Secured password", [222, 122, 181, 154, 106, 55, 132, 77, 233, 139, 9, 254, 71, 5, 161, 215], [13, 80, 233, 146, 255, 115, 143, 118, 151, 220, 183, 180, 113, 119, 43, 159, 164, 224, 121, 75, 103, 233, 252, 159, 108, 53, 127, 51, 22, 222, 165, 86]));
    passwords.push(("2DXq9JYvMZHg7swL6nU3Ai5rKo0Bt1RpS8WTz4QcGFekdbIxClhfmaPEjOqNuVynXZGb9Hj8K2Y5A3qL7D0MtOu4crF1EpSixWIznvlPwQeTaJgCkUBmdRhoysLxfV3u", [22, 165, 237, 215, 141, 194, 165, 222, 251, 34, 242, 175, 132, 184, 123, 119], [11, 250, 105, 73, 127, 76, 96, 162, 136, 22, 200, 237, 21, 124, 100, 2, 237, 205, 180, 151, 246, 45, 127, 39, 140, 104, 17, 40, 229, 149, 40, 100]));
    passwords.push(("LA$$14NdbUpixfE", [29, 78, 26, 135, 236, 250, 16, 54, 16, 237, 83, 24, 82, 212, 66, 64], [79, 118, 64, 213, 13, 252, 174, 49, 152, 6, 165, 29, 64, 0, 177, 67, 176, 182, 130, 9, 177, 81, 147, 121, 216, 174, 239, 75, 43, 215, 55, 6]));
    passwords.push(("123", [93, 85, 127, 70, 40, 79, 213, 227, 208, 234, 164, 171, 137, 214, 168, 13], [18, 6, 191, 207, 9, 30, 134, 152, 202, 229, 119, 238, 188, 84, 28, 147, 170, 123, 88, 26, 38, 235, 221, 134, 9, 187, 180, 86, 10, 40, 7, 137]));
    passwords.push(("♕", [5, 190, 93, 36, 80, 144, 194, 51, 197, 197, 80, 41, 168, 13, 89, 66], [48, 100, 112, 159, 27, 39, 33, 223, 189, 233, 89, 160, 219, 246, 212, 22, 137, 216, 220, 28, 163, 40, 143, 17, 109, 56, 145, 128, 46, 134, 111, 63]));
    passwords.push((" ", [30, 151, 79, 203, 50, 9, 166, 43, 156, 163, 218, 152, 140, 75, 49, 61], [177, 124, 153, 40, 116, 125, 33, 162, 186, 60, 141, 54, 115, 127, 73, 60, 64, 80, 180, 249, 111, 102, 77, 240, 220, 132, 184, 229, 85, 251, 129, 136]));
    passwords.push(("😎", [43, 12, 195, 39, 32, 210, 12, 196, 158, 128, 98, 57, 175, 155, 172, 247], [212, 41, 189, 193, 239, 50, 130, 164, 109, 250, 43, 9, 71, 2, 204, 182, 185, 230, 181, 51, 145, 139, 62, 91, 187, 186, 246, 113, 136, 226, 54, 205]));

    for (password, salt, expected_key) in  passwords.iter() {
        let generated_key = Crypto::create_key_from_password(password, salt);
        assert_eq!(generated_key, *expected_key, "Failed for password: {}", password);
    }

}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MyStruct {
    field1: String,
    field2: i32,
}

#[test]
  fn serialize_and_encrypt_struct() {
    
    let data = MyStruct {
        field1: "Hello, World!".to_string(),
        field2: 42,
    };
    let serialized = serde_json::to_vec(&data).unwrap();

    let key: [u8; 32] = [13, 80, 233, 146, 255, 115, 143, 118, 151, 220, 183, 180, 113, 119, 43, 159, 164, 224, 121, 75, 103, 233, 252, 159, 108, 53, 127, 51, 22, 222, 165, 86];
    let encrypted_data = Crypto::encrypt_for_storage(&serialized, &key);

    let decrypted_data = Crypto::decrypt_from_storage(&encrypted_data, &key);

    match decrypted_data {
      Ok(v) => {
          let clear_data: MyStruct = serde_json::from_slice(&v).unwrap();
          assert_eq!(clear_data, data); 
      },
      Err(_) => panic!("Decryption failed"),
    }
  }
