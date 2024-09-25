// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod entities;
mod encryption;
mod utils;
mod error;

use crate::encryption::*;
use crate::entities::*;
use crate::crypto::*;
use entities::Result;
use tauri::Manager;

// Learn more about Tauri commands at v
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_profil(database: tauri::State<Database>, name: &str, password: &str) -> Result<()>{
   let _ = MasterProfil::create_store_in_db(name, password, &database.db)?;
   Ok(())
}

#[tauri::command]
fn verify_profil(database: tauri::State<Database>, name: &str, password: &str) -> Result<()>{
   let user = MasterProfil::get_valide_existing_user(name, password, &database.db)?;
   Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
          let database = Database::init(r"stored/data.db")?;
          app.manage(database);
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_profil, verify_profil])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
