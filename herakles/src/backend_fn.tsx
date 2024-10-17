import { invoke } from "@tauri-apps/api/tauri";

export async function add_profil(name: string, password: string) {
  try {
    await invoke("add_profil", {name, password});
    console.log("add profil succefully");
  }
  catch (error) {
    throw(error)
  }
}


export async function connect_profil(name: string, password: string) {
  try {
    await invoke("verify_profil", {name, password});
    console.log("User is valide");
  }
  catch (error) {
    throw(error)
  }
}

export async function get_vault_by_id(user_id: number) {
  try {
    await invoke("get_vault_by_id", {user_id});
    console.log("get_vault_by_id");
  }
  catch (error) {
    throw(error)
  }
}

export async function add_vault(user_id: number, name:string) {
  try {
    await invoke("create_vault", {user_id:user_id, name:name});
    console.log("create_vault");
  }
  catch (error) {
    throw(error)
  }
}
