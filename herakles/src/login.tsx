import React, {useState, useEffect} from 'react';
import { NextUIProvider, Select, SelectItem , Input, Spacer, Card, CardFooter, Image, Button} from '@nextui-org/react';

import "./App.css";

const Login = () => {
  const [isVisible, setIsVisible] = React.useState(false);

  const [loginCardsList, setLoginCardsList] = useState([
    { id: '1', imageSrc: "./media/img/locker2.svg", name: "Johnny"},
    { id: '2', imageSrc: "./media/img/locker2.svg", name: "Frank"},
    { id: '3', imageSrc: "./media/img/locker2.svg", name: "Max"}
  ]);

  const loginCards = loginCardsList.map((card, index) => (
    <Card
      isFooterBlurred
      isPressable
      radius="lg"
      className="border-none m-3"
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
  ));

  return (
    <>
    <div className="w-full items-center flex flex-col">
          <h1 className="mb-4 text-8xl font-extrabold leading-none tracking-tight text-gray-900 md:text-8xl lg:text-8xl dark:text-white">LOGIN</h1>
        </div>
    <div className='flex flex-row justify-center items-center flex-wrap mt-4'>
      {loginCards}
    </div>
    </>
  );
};

export default Login;