import React, {useState, useEffect} from 'react';
import { NextUIProvider, Card, Skeleton , Chip, ScrollShadow} from '@nextui-org/react';
import {Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, useDisclosure, Input, Textarea} from "@nextui-org/react";
import {Image} from "@nextui-org/image";
import img_locker from './media/img/locker2.svg';
import img_edit from './media/img/edit1.svg';
import SimpleBar from 'simplebar-react';
import 'simplebar-react/dist/simplebar.min.css';
import './App.css';

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
    <Card className="w-[400px] space-y-5 p-4 m-2" radius="lg" key={index}>
      <Skeleton className="rounded-lg">
        <div className="h-24 rounded-lg bg-default-300"></div>
      </Skeleton>
      <div className="space-y-3">
        <Skeleton className="w-3/5 rounded-lg">
          <div className="h-3 w-3/5 rounded-lg bg-default-200"></div>
        </Skeleton>
        <Skeleton className="w-4/5 rounded-lg">
          <div className="h-3 w-4/5 rounded-lg bg-default-200"></div>
        </Skeleton>
        <Skeleton className="w-2/5 rounded-lg">
          <div className="h-3 w-2/5 rounded-lg bg-default-300"></div>
        </Skeleton>
      </div>
    </Card>
  ));

  const vaultsCards = vaultsCardsList.map((card, index) => (
    <Card isPressable className="w-[400px] space-y-5 m-2 vault-container" radius="lg" key={card.id}>
      <div className='w-full flex flex-row items-center p-4 vault-container-top'>
        <img width={60} alt={card.title} src={img_locker}></img>
        <div className='w-full flex flex-col justify-between ml-4 pl-1' style={{ height: '64px' }}>
          <h2 className="flex text-2xl md:text-2xl lg:text-2xl font-extrabold leading-none tracking-tight text-gray-900dark:text-white" style={{ fontWeight: '600' }}>
            {card.title}
          </h2>
          <div className='w-full flex flex-row items-center'>
            <Chip color="warning" variant="flat"><p>{card.itemCount}</p></Chip> 
            <p className="text-1xl md:text-1xl lg:text-1xl leading-none tracking-tight text-gray-900dark:text-white pl-1.5" style={{ fontWeight: '400' }}>Items</p>
          </div>
        </div>
        <div className="h-full" style={{ height: 64 }}>
          <img width={50} alt={card.title} src={img_edit} className='vault-container-editor-icon' onClick={() => handleOpen(card.id)} />
        </div>
      </div>
      <div className='w-full flex px-2'>
        <SimpleBar data-simplebar-auto-hide="false" className='w-full flex flex-col items-start px-2' style={{ height: 70 }}>
          <p className='flex' style={{ color: 'hsl(37 91% 55% / 1)' }}>Description:</p>
          {card.description !== "" ? (
            <p className='flex ml-0.5 text-left'>{card.description}</p>
          ) : (
            <p className='flex ml-0.5 text-left' style={{ color: 'hsl(37 91% 55% / .2)' }}>None</p>
          )}
        </SimpleBar>
      </div>
      <div className="space-y-3">

        </div>
    </Card>
  ));

  return (
    <>
      <Modal isOpen={isOpen} onOpenChange={onOpenChange} onClose={onClose}>
        <ModalContent>
          <ModalHeader className="flex flex-col gap-1">Edit vault</ModalHeader>
          <ModalBody>
            <Input label="Title" value={modalTitle} onChange={(e) => setModalTitle(e.target.value)} />
            <Textarea label="Description" value={modalDesc} onChange={(e) => setModalDesc(e.target.value)} placeholder="Enter your description" />
          </ModalBody>
          <ModalFooter>
            <Button color="danger" variant="light" onPress={onClose}>Cancel</Button>
            <Button color="primary" onPress={handleSaveChanges}>Save</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
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