// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod entities;
mod encryption;
mod utils;
mod error;

use crate::encryption::*;
use crate::entities::*;
use crate::crypto::*;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn add_profil(database: tauri::State<Database>) {
//   MasterProfil::create_store_in_db("test", "ouiouioui", &database.db);
// }



fn main() {
    tauri::Builder::default()
        .setup(|app| {
          let database = Database::init(r"stored/data.db")?;
          //app.manage(database);
          Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
