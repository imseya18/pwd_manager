#[cfg(test)]

use crate::MasterProfil;
use crate::entities::traits::Insertable;
use rusqlite::Connection;
use bcrypt::{hash, verify, DEFAULT_COST};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?; // Ouvre une base de données en mémoire
    conn.execute_batch(
      "CREATE TABLE if not exists master_profil (
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
        62.to_string(),
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
    62.to_string(),
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
  let new_profil = MasterProfil::create_store_new_profil("1234", "JGLP", "c'est ok", &db).expect("failed to create profil");
  let get_profil = MasterProfil::get_by_name("JGLP".to_string(), &db).expect("probleme");

  assert_eq!(new_profil.name, get_profil.name);
  assert_eq!(new_profil.uid, get_profil.uid);
  assert_eq!(new_profil.master_password, get_profil.master_password);
}

#[test]
fn get_master_profil_from_db_error(){
  let db = setup_test_db().expect("failed to connect to db");
  let get_profil = MasterProfil::get_by_name("JGLP".to_string(), &db);
  assert_eq!(get_profil.is_ok(), false);
}

#[test]
fn good_user_good_password() -> Result<()>{
  let db = setup_test_db().expect("failed to connect to db");
  let insert_profil = MasterProfil::create_store_new_profil(
    "uid",
    "JGLP2", "1234",
    &db)?;
  let get_profil = MasterProfil::get_valide_existing_user("JGLP2", "1234", &db);
  assert!(get_profil.is_ok(), "Expected no error due to good password");
  Ok(())
}

#[test]
fn good_user_wrong_password() -> Result<()>{
  let db = setup_test_db().expect("failed to connect to db");
  let insert_profil = MasterProfil::create_store_new_profil(
    "uid",
    "JGLP2", "1234",
    &db)?;
  let get_profil = MasterProfil::get_valide_existing_user("JGLP2", "dfsdgdg", &db);
  assert!(get_profil.is_err(), "Expected an error due to wrong password");
  Ok(())
}
