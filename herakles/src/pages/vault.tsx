import React, {useState, useEffect} from 'react';
import { NextUIProvider, Card, Skeleton , Chip, ScrollShadow} from '@nextui-org/react';
import {Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, useDisclosure, Input, Textarea} from "@nextui-org/react";
import {Image} from "@nextui-org/image";
import img_locker from '../media/img/locker2.svg';
import img_edit from '../media/img/edit1.svg';
import SimpleBar from 'simplebar-react';
import 'simplebar-react/dist/simplebar.min.css';
import '../App.css';

import VaultModal from './vault/VaultModal.tsx'
import VaultCard from './vault/VaultCard.tsx'
import VaultSkeleton from './vault/VaultSkeleton.tsx';

const Vault = () => {
  const { isOpen, onOpen, onOpenChange , onClose} = useDisclosure();
  const [size, setSize] = useState('md');
  const [modalTitle, setModalTitle] = useState('');
  const [modalDesc, setModalDesc] = useState('');
  const [currentId, setCurrentId] = useState('');

  const [vaultsCardsList, setVaultsCardsList] = useState([
    { id: '1', imageSrc: "./media/img/locker2.svg", title: "Administratif", itemCount: 512, description: "Ce coffre-fort contient des informations critiques pour la gestion et l'administration de l'infrastructure informatique" },
    { id: '2', imageSrc: "./media/img/locker2.svg", title: "Technique", itemCount: 307, description: "" },
    { id: '3', imageSrc: "./media/img/locker2.svg", title: "Opérations", itemCount: 150, description: "Description de la carte 3." }
  ]);

  const handleOpen = (id:string) => {
    const vault = vaultsCardsList.find(v => v.id === id);
    if (!vault) return;
    setCurrentId(id);
    setModalTitle(vault.title);
    setModalDesc(vault.description);
    setSize("sm");
    onOpen();
  };

  const handleSaveChanges = () => {
    const updatedList = vaultsCardsList.map(vault => {
      if (vault.id === currentId) {
        return { ...vault, title: modalTitle, description: modalDesc };
      }
      return vault;
    });
    setVaultsCardsList(updatedList);
    onClose();
    //onOpenChange(false); // or whatever method you use to close the modal
  };

  useEffect(() => {
      

    function adjustContainerWidth() {
      const container = document.getElementById('container_vault');
      if (!container) return;

      if (window.innerWidth > 1300) {
        container.style.width = '1248px';
      } else if (window.innerWidth > 900) {
        container.style.width = '832px';
      } else {
        container.style.width = '416px';
      }
    }

    adjustContainerWidth();

    window.addEventListener('resize', adjustContainerWidth);


    return () => {
      window.removeEventListener('resize', adjustContainerWidth);
    };
  }, []);

  const skeletonCards = Array.from({ length: 4 }).map((_, index) => (
    <VaultSkeleton/>
  ));

  const vaultsCards = vaultsCardsList.map((card, index) => (
    <VaultCard
          key={card.id}
          card={card}
          img_locker={img_locker}
          img_edit={img_edit}
          handleOpen={handleOpen}
        />
  ));

  

  return (
    <>
      <VaultModal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        onClose={onClose}
        modalTitle={modalTitle}
        setModalTitle={setModalTitle}
        modalDesc={modalDesc}
        setModalDesc={setModalDesc}
        handleSaveChanges={handleSaveChanges}
      />
      <div className="flex gap-4 items-center flex-col w-full" style={{ padding: '20px', margin: 'auto' }}>
        <div className="w-full items-center flex flex-col">
          <h1 className="mb-4 text-8xl font-extrabold leading-none tracking-tight text-gray-900 md:text-8xl lg:text-8xl dark:text-white">VAULTS</h1>
        </div>
        <div className="w-full items-center flex flex-row flex-wrap" id="container_vault">
          {vaultsCards}

        </div>
      </div>
    </>
  );
};

export default Vault;