import { Button } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import "./App.css";
import logo from "./media/img/logo_transparent.svg";
function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-4">
      <div className="flex flex-col items-center justify-center">
        <img src={logo} className="w-[300px]"></img>
        <p className="SFMono-Regular font-bold text-4xl">HERAKLES</p>
      </div>
      <div className="flex flex-row gap-4">
      <Button size="lg" className="w-[200px] hover:bg-gradient-to-tr from-orange-500 to-red-500">
        Connexion
      </Button>      <Button size="lg" className="w-[200px] hover:bg-gradient-to-tr from-orange-500 to-red-500">
          Add Profil
      </Button>
      <Button size="lg" className="w-[200px] hover:bg-gradient-to-tr from-orange-500 to-red-500">
          Add Vault
      </Button>
      <Button size="lg" className="w-[200px] hover:bg-gradient-to-tr from-orange-500 to-red-500">
            Add account
      </Button>
      </div>
    </div>
  );
}

export default App;
