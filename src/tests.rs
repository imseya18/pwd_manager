#[cfg(test)]

use crate::MasterProfil;
use crate::entities::traits::Insertable;
use rusqlite::{Connection, Result};

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?; // Ouvre une base de données en mémoire
    conn.execute_batch(
      "CREATE TABLE if not exists master_profil (
          id_profil INTEGER PRIMARY KEY AUTOINCREMENT,
          uid_profil TEXT not null unique,
          name VARCHAR(15) not null unique,
          master_password VARCHAR not null);

      CREATE TABLE if not exists vault (
          id_vault INTEGER PRIMARY KEY AUTOINCREMENT,
          id_profil INTEGER REFERENCES master_profil (id_profil) not null,
          uid_vault TEXT not null unique,
          name VARCHAR(15) not null,
          created_at TIMESTAMP not null,
          updated_at TIMESTAMP);

      CREATE TABLE if not exists account (
          id_account INTEGER PRIMARY KEY AUTOINCREMENT,
          id_vault INTEGER REFERENCES vault (id_vault) not null,
          uid_account TEXT not null unique,
          name VARCHAR(20) not null,
          label VARCHAR(20),
          account_name VARCHAR(20) not null,
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
        "test_hash".to_string(),
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
  assert!(conn.is_ok());
}
