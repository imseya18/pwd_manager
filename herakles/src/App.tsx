import { Button } from "@nextui-org/react";

import { useState } from "react";
import "./App.css";
import LoginModal from "./components/Modal";
import logo from "./media/img/logo_transparent.svg";



function App() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 `${isModalOpen ? 'filter blur-sm' : ''}">
      <div className="flex flex-col items-center justify-center">
        <img src={logo} className="w-[300px]"></img>
        <p className="SFMono-Regular font-bold text-4xl">HERAKLES</p>
      </div>
      <div className="flex flex-row gap-4">
      <LoginModal />
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
