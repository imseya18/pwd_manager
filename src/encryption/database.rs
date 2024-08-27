use rusqlite::{Connection, Result};
use std::fs::{self, File};
use std::error::Error;
use std::path::{PathBuf, Path};

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{Rng, rngs::OsRng};
use std::io::{self, Write, Read};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Debug)]
pub struct Database {
    pub path: String,
    pub db: Connection,
}

impl  Database {

    /*
        Take database path and create folder of db path if not exist
    */
    pub fn init(path: &str) -> Result<Self, Box<dyn Error>> {
        let db_path = PathBuf::from(path);

        if let Some(dir_path) = db_path.parent()
        {
            if !dir_path.exists()
            {
                fs::create_dir_all(dir_path)?;
            }

        }
        let db_path_str = db_path.to_str().ok_or_else(|| {
                rusqlite::Error::InvalidPath(db_path.clone())
        })?;

        Ok(Self::connect(db_path_str)?)
    }


    /*
        Connection with sqlite db
    */
    pub fn connect(db_path_str: &str) -> Result<Self> {
        let conn = Connection::open(db_path_str)?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;

            CREATE TABLE if not exists master_profil (
                id_profil INTEGER PRIMARY KEY AUTOINCREMENT,
                uid_profil TEXT not null unique,
                name TEXT not null unique,
                master_password TEXT not null);

            CREATE TABLE if not exists vault (
                id_vault INTEGER PRIMARY KEY AUTOINCREMENT,
                id_profil INTEGER REFERENCES master_profil (id_profil) ON DELETE CASCADE not null,
                uid_vault TEXT not null unique,
                name BLOB not null,
                created_at TIMESTAMP not null,
                updated_at TIMESTAMP);

            CREATE TABLE if not exists account (
                id_account INTEGER PRIMARY KEY AUTOINCREMENT,
                id_vault INTEGER REFERENCES vault (id_vault) ON DELETE CASCADE not null,
                uid_account TEXT not null unique,
                sensitive_data BLOB not null,
                created_at BLOB not null,
                updated_at BLOB not null)"
        )?;
        //self.db = Some(conn);
        Ok(Database { path: db_path_str.to_string(), db: conn })
    }


    pub fn is_encrypted(&self) -> io::Result<bool> {
        let path = Path::new(&self.path);
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 16];
        file.read_exact(&mut buffer)?;

        Ok(buffer != *b"SQLite format 3\0")
    }


    pub fn encrypt_file(&self, path: String, key: &[u8]) -> Result<bool, Box<dyn Error>> {
        //if self.is_encrypted()? {
        //    return Ok(false);
        //}

        let input_path = Path::new(&path);
        let output_path = Path::new(&path);

        let mut iv = [0u8; 16]; // 128 bits pour AES
        OsRng.fill(&mut iv);

        let data = fs::read(input_path)?;

        let cipher = Aes256Cbc::new_from_slices(key, &iv).expect("Error creating cipher");

        let encrypted_data = cipher.encrypt_vec(&data);

        let mut output_file = File::create(output_path)?;

        output_file.write_all(&iv)?;
        output_file.write_all(&encrypted_data)?;

        Ok(true)
    }

    pub fn decrypt_file(&self, path: String, key: &[u8]) -> Result<bool, Box<dyn Error>> {
        //if !self.is_encrypted()? {
        //    return Ok(false);
        //}

        let input_path = Path::new(&path);
        let output_path = Path::new(&path);

        let mut file = File::open(input_path)?;
        let mut iv = [0u8; 16];
        file.read_exact(&mut iv)?;

        let mut encrypted_data = Vec::new();
        file.read_to_end(&mut encrypted_data)?;

        let cipher = Aes256Cbc::new_from_slices(key, &iv).expect("Error creating cipher");

        let decrypted_data = cipher.decrypt_vec(&encrypted_data)?;

        let mut output_file = File::create(output_path)?;
        output_file.write_all(&decrypted_data)?;

        Ok(true)
    }

}
