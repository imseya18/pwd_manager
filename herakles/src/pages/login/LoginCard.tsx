import React from 'react';
import { Card, CardFooter, Image } from '@nextui-org/react';


const LoginCard = ({ card, onClick }) => {
  return (
    <Card
      isFooterBlurred
      isPressable
      radius="lg"
      className="border-none m-3"
      onClick={onClick}
    >
    <div className='w-full flex items-center justify-center' style={{height:165, width:200}}>
      <Image
        alt="Profiles"
        className="object-cover"
        height={120}
        src={card.imageSrc}
      />
    </div>
      <CardFooter className="max-h-8 flex items-center justify-center">
        <p className='text-1xl leading-none tracking-tight text-gray-00 md:text-1xl lg:text-1xl dark:text-white'>{card.name}</p>
      </CardFooter>
    </Card>
  );
};

export default LoginCard;
