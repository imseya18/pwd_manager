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
