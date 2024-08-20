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
    pub fn path(path: &str) -> Result<String, Box<dyn Error>> {
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
        Ok(db_path_str.to_string())
    }


    /*
        Connection with sqlite db
    */
    pub fn init(path: String) -> Result<Self> {
        let conn = Connection::open(&path)?;
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
                updated_at TIMESTAMP)"
        )?;
        Ok(Database { path, db: conn })
    }


    fn is_encrypted(&self) -> io::Result<bool> {
        let path = Path::new(&self.path);
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 16];
        file.read_exact(&mut buffer)?;
    
        Ok(buffer != *b"SQLite format 3\0")
    }


    pub fn encrypt(&self, key: &[u8]) -> Result<bool, Box<dyn Error>> {
        if self.is_encrypted()?
        {
            return Ok(false);
        }

        let input_path = Path::new(&self.path);
        let output_path = Path::new(&self.path);

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

    pub fn decrypt()
    {

    }
}
