import { Button } from "@nextui-org/react";

import { useState } from "react";
import "./App.css";
import { add_profil, connect_profil } from "./backend_fn.tsx";
import LoginModal from "./components/Modal";
import logo from "./media/img/logo_transparent.svg";


function App() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  console.log("add_profil is: ", typeof add_profil);
  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 `${isModalOpen ? 'filter blur-sm' : ''}">
      <div className="flex flex-col items-center justify-center">
        <img src={logo} className="w-[300px]"></img>
        <p className="SFMono-Regular font-bold text-4xl">HERAKLES</p>
      </div>
      <div className="flex flex-row gap-4">
      <LoginModal buttonLabel="Sign in" onSignIn={connect_profil}/>
      <LoginModal buttonLabel="Register" onSignIn={add_profil}/>
        <Button size="lg" className="btn-custom">
            Add Profil
        </Button>
        <Button size="lg" className="btn-custom">
            Add Vault
        </Button>
        <Button size="lg" className="btn-custom">
            Add account
        </Button>
      </div>
    </div>
  );
}

export default App;
