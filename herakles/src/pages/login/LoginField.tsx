import React from 'react';
import { Input, Button} from '@nextui-org/react';

const LoginField = ({ account, onClose}) => {
  return (
    <div className='w-full items-center flex flex-col'>
    <div className='w-[400px] space-y-5 m-2 p-5 flex flex-col items-center mt-5'>
        <p className='text-2xl leading-none tracking-tight text-gray-900 md:text-2xl lg:text-2xl dark:text-white pl-1 pb-2'>{account.name}</p>
        <Input
        autoFocus
        label="Password"
        placeholder="Enter your password"
        type="password"
        variant="bordered"
        />
        <div className='w-full flex flex-row justify-end'>
        <Button color="danger" variant="flat" className='mr-1'  onClick={onClose}>
            Close
        </Button>
        <Button color="primary">
            Sign in
        </Button>
        </div>
    </div>
    </div>
    );
};

export default LoginField;