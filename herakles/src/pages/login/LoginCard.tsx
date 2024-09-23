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
      <Image
        alt="Profiles"
        className="object-cover"
        height={165}
        src="https://nextui.org/images/card-example-6.jpeg"
        width={200}
      />
      <CardFooter className="max-h-8 flex items-center justify-center">
        <p className='text-1xl leading-none tracking-tight text-gray-00 md:text-1xl lg:text-1xl dark:text-white'>{card.name}</p>
      </CardFooter>
    </Card>
  );
};

export default LoginCard;
