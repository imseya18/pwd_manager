import { Button } from "@nextui-org/react";
import { useState } from "react";
import "./App.css";
import { add_vault, get_vault_by_id } from "./backend_fn";
import img_edit from './media/img/edit1.svg';
import img_locker from './media/img/locker2.svg';
import logo from "./media/img/logo_transparent.svg";
import { VaultCard } from './pages/vault/VaultCard.tsx';
import { VaultResult } from "./types/vault";

function SuccessLogin() {
  let vault_number = 1;
  const [vaults, Setvaults] = useState<VaultResult[]>([]);
  const [showVaults, setShowVaults] = useState(false);

  const handleGetVault = async () => {
    try {
      const vaultsData = await get_vault_by_id(22);
      Setvaults(vaultsData);
      setShowVaults(true);
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
      {!showVaults ?
        (
          <>
          <Button onPress={addVault} size="lg" className="btn-custom">
            ADD VAULT
          </Button>
          <Button onPress={handleGetVault} size="lg" className="btn-custom">
            GET VAULT
          </Button>
          </>
          ) : (
            <div className="w-full items-center flex flex-row flex-wrap" id="container_vault">
              {
              vaults.map((vault) => (
              vault.success ?(
                <VaultCard
                  key={vault.success.uid}
                  card={vault.success}
                  img_locker={img_locker}
                  img_edit={img_edit}
                  handleOpen={() => null}
              />) : null))}
            </div>
          )

      }
    </div>
  );
}

export default SuccessLogin;
