import React from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Input, Textarea, Button } from '@nextui-org/react';

const VaultModal = ({ 
  isOpen, 
  onOpenChange, 
  onClose, 
  modalTitle, 
  setModalTitle, 
  modalDesc, 
  setModalDesc, 
  handleSaveChanges 
}) => {
  return (
    <Modal isOpen={isOpen} onOpenChange={onOpenChange} onClose={onClose}>
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">Edit vault</ModalHeader>
        <ModalBody>
          <Input 
            label="Title" 
            value={modalTitle} 
            onChange={(e) => setModalTitle(e.target.value)} 
          />
          <Textarea 
            label="Description" 
            value={modalDesc} 
            onChange={(e) => setModalDesc(e.target.value)} 
            placeholder="Enter your description" 
          />
        </ModalBody>
        <ModalFooter>
          <Button color="danger" variant="light" onPress={onClose}>
            Cancel
          </Button>
          <Button color="primary" onPress={handleSaveChanges}>
            Save
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export default VaultModal;
