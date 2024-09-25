import {
  Button,
  Input,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  useDisclosure
} from '@nextui-org/react';
import React, { useRef } from 'react';
import { LockIcon } from './LockIcon.jsx'; // Chemin vers vos icônes
import { MailIcon } from './MailIcon.jsx';

interface LoginModalProps {
  buttonLabel?: string; // Label du bouton (par défaut : "Connexion")
  onSignIn:(accountName: string, password: string) => Promise<void> | void; // Fonction passée en prop pour l'action à exécuter sur "Sign in"
}

const LoginModal: React.FC<LoginModalProps> = ({ buttonLabel = "Connexion", onSignIn }) => {
  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const accountNameRef = useRef(null);
  const passwordRef = useRef(null);
  console.log("onSignIn is:", typeof onSignIn);

  const handleSignIn = async () => {
    const accountName = accountNameRef.current.value;
    const password = passwordRef.current.value;

    if (!accountName || !password) {
      console.log("Please fill in all fields.");
      return;
    }

    try {
      await onSignIn(accountName, password);
      onOpenChange();
    }
    catch(error){
      console.error("Erreur lors de l'ajout du profil :", error);
    }
   };

  return (
    <>
      {/* Bouton pour ouvrir le Modal */}
      <Button onPress={onOpen} className='btn-custom h-[48px]'>
        {buttonLabel}
      </Button>

      {/* Modal */}
      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        placement="top-center"
      >
        <ModalContent>
          {(close) => (
            <>
              <ModalHeader className="flex flex-col gap-1">Log in</ModalHeader>
              <ModalBody>
                <Input
                  autoFocus
                  endContent={
                    <MailIcon className="text-2xl text-default-400 pointer-events-none flex-shrink-0" />
                  }
                  label="Account Name"
                  placeholder="Enter your Name Account"
                  variant="bordered"
                  ref={accountNameRef}
                />
                <Input
                  endContent={
                    <LockIcon className="text-2xl text-default-400 pointer-events-none flex-shrink-0" />
                  }
                  label="Password"
                  placeholder="Enter your password"
                  type="password"
                  variant="bordered"
                  ref={passwordRef}
                />
              </ModalBody>
              <ModalFooter>
                <Button color="danger" variant="flat" onPress={close}>
                  Close
                </Button>
                <Button color="primary" onPress={handleSignIn}>
                  Sign in
                </Button>
              </ModalFooter>
            </>
          )}
        </ModalContent>
      </Modal>
    </>
  );
};

export default LoginModal;
