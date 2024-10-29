import { Button } from "@nextui-org/react";

import { useState } from "react";
import "./App.css";
import { add_profil, connect_profil } from "./backend_fn.tsx";
import LoginModal from "./components/Modal";
import logo from "./media/img/logo_transparent.svg";

const modalTypes = {
  login: {
    onSignIn: connect_profil,
  },
  register: {
    onSignIn: add_profil
  }
}

function Loging({setIsLogin}) {
  const [modalType, setModalType] = useState<null | string>(null);

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 `${isModalOpen ? 'filter blur-sm' : ''}">
      <div className="flex flex-col items-center justify-center">
        <img src={logo} className="w-[300px]"></img>
        <p className="SFMono-Regular font-bold text-4xl">HERAKLES</p>
      </div>
      <div className="flex flex-row gap-4">
        <Button onPress={() => setModalType('login')} size="lg" className="btn-custom">
          Sign in
        </Button>
        <Button onPress={() => setModalType('register')} size="lg" className="btn-custom">
          register
        </Button>
        {modalType && (<LoginModal
          isOpen={true}
          onClose={() => setModalType(null)}
          onSignIn={modalTypes[modalType].onSignIn}
          setIsLogin={setIsLogin}
        />)}
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

export default Loging;
