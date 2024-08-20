use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Database {
    pub path: String,
    pub db: Connection,
}
  
impl  Database {

    pub fn init(path: String) -> Result<Self> {
        let conn = Connection::open(&path)?;
        conn.execute_batch(
            "CREATE TABLE if not exists master_profil (
                id_profil VARCHAR primary key,
                name VARCHAR(15) not null unique,
                master_password VARCHAR not null);
      
            CREATE TABLE if not exists vault (
                id_vault VARCHAR primary key,
                id_profil VARCHAR REFERENCES master_profil (id_profil) not null,
                name VARCHAR(15) not null,
                created_at TIMESTAMP not null,
                updated_at TIMESTAMP);
      
            CREATE TABLE if not exists account (
                id_account VARCHAR primary key,
                id_vault VARCHAR REFERENCES vault (id_vault) not null,
                name VARCHAR(20) not null,
                label VARCHAR(20),
                account_name VARCHAR(20) not null,
                password TEXT not null,
                url TEXT,
                note TEXT,
                created_at TIMESTAMP not null,
                updated_at TIMESTAMP)"
        )?;
        Ok(Database { path, db: conn })
    }
}
  