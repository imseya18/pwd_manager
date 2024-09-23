import React from 'react';
import { Card, Chip } from '@nextui-org/react';
import SimpleBar from 'simplebar-react';
import 'simplebar-react/dist/simplebar.min.css';

const VaultCard = ({ card, img_locker, img_edit, handleOpen }) => {
  return (
    <Card isPressable className="w-[400px] space-y-5 m-2 vault-container" radius="lg" key={card.id}>
      <div className='w-full flex flex-row items-center p-4 vault-container-top'>
        <img width={60} alt={card.title} src={img_locker} />
        <div className='w-full flex flex-col justify-between ml-4 pl-1' style={{ height: '64px' }}>
          <h2 className="flex text-2xl md:text-2xl lg:text-2xl font-extrabold leading-none tracking-tight text-gray-900 dark:text-white" style={{ fontWeight: '600' }}>
            {card.title}
          </h2>
          <div className='w-full flex flex-row items-center'>
            <Chip color="warning" variant="flat"><p>{card.itemCount}</p></Chip> 
            <p className="text-1xl md:text-1xl lg:text-1xl leading-none tracking-tight text-gray-900 dark:text-white pl-1.5" style={{ fontWeight: '400' }}>Items</p>
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
  );
};

export default VaultCard;
