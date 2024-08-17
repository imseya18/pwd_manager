use rusqlite::{Connection, Result};

fn init_database() -> Result<Connection>{
  let conn = Connection::open("test.db")?;
  conn.execute_batch(
  "CREATE TABLE if not exists master_profil (
       id_profil INTEGER primary key,
       name VARCHAR(15) not null unique,
       master_password VARCHAR not null);

       CREATE TABLE if not exists vault (
       id_vault INTEGER primary key,
       id_profil INTEGER REFERENCES master_profil (id_profil) not null,
       name VARCHAR(15) not null,
       created_at TIMESTAMP not null,
       updated_at TIMESTAMP
       );

       CREATE TABLE if not exists account (
       id_account INTEGER primary key,
       id_vault INTEGER REFERENCES vault (id_vault) not null,
       name VARCHAR(20) not null,
       label VARCHAR(20),
       account_name VARCHAR(20) not null,
       password TEXT not null,
       url TEXT,
       note TEXT,
       created_at TIMESTAMP not null,
       updated_at TIMESTAMP
       )")?;
  Ok(conn)
}

fn main() -> Result<()> {
  let db = match init_database() {
    Ok(db) => db,
    Err(e) => return Err(e),
  };
  db.execute("INSERT INTO master_profil (id_profil, name, master_password) values (72, 'ouiouioui', 'qwertyopjdfg')", ())?;
  Ok(())
}
