import { Button } from "@nextui-org/react";
import "./App.css";
import { add_vault, get_vault_by_id } from "./backend_fn";
import logo from "./media/img/logo_transparent.svg";

function SuccessLogin() {
  console.log('SuccesLogin page');
  let vault_number = 1;
  const handleGetVault = async () => {
    try {
      await get_vault_by_id(22);
    }
    catch(error){
      console.error("erreur while trying to GET Vault", error);
    }
  }

  const addVault = async () => {
    try {
      let vault_name = "TestVault" + vault_number;
      vault_number += 1;
      await add_vault(22, vault_name);
    }
    catch(error){
      console.error("erreur while trying to ADD Vault", error);
    }
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-8 `${isModalOpen ? 'filter blur-sm' : ''}">
      <div className="flex flex-col items-center justify-center">
        <img src={logo} className="w-[300px]"></img>
        <p className="SFMono-Regular font-bold text-4xl">HERAKLES</p>
      </div>
      <Button onPress={addVault} size="lg" className="btn-custom">
        ADD VAULT
      </Button>
      <Button onPress={handleGetVault} size="lg" className="btn-custom">
        GET VAULT
      </Button>
    </div>
  );
}

export default SuccessLogin;
