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
use vault::Vault;
use crate::vault::VaultResult;

#[tauri::command(rename_all = "snake_case")]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn add_profil(database: tauri::State<Database>, name: &str, password: &str) -> Result<()>{
   let _ = MasterProfil::create_store_in_db(name, password, &database.db)?;
   Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn verify_profil(database: tauri::State<Database>, name: &str, password: &str) -> Result<()>{
   let user = MasterProfil::get_valide_existing_user(name, password, &database.db)?;
   Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn get_vault_by_id(database: tauri::State<Database>, user_id: i64) -> Result<Vec<VaultResult>>{
  let vaults = Vault::get_by_user_id(user_id, &database.db)?;
  let vaults_response:Vec<VaultResult> = vaults.into_iter()
                        .map(|res| match res {
                          Ok(vault) => VaultResult {
                              success: Some(vault),
                              error: None,
                            }
                          ,
                          Err(e) => VaultResult {
                                success: None,
                                error: Some(e.to_string()),
                              },
                        })
                        .collect();
  for vault in vaults_response.iter(){
    println!("vault = {:?}", vault.success);
  }
  Ok(vaults_response)
}

#[tauri::command(rename_all = "snake_case")]
fn create_vault(database: tauri::State<Database>, user_id: i64, name: &str) -> Result<()>{
  println!("{:?}", user_id);
  let _ = Vault::create_store_in_db(user_id, name, &database.db)?;
  Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
          let database = Database::init(r"stored/data.db")?;
          app.manage(database);
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_profil, verify_profil, get_vault_by_id, create_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
