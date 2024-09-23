import React, {useState, useEffect} from 'react';
import { NextUIProvider, Select, SelectItem , Input, Spacer, Card, CardFooter, Image, Button} from '@nextui-org/react';
import '../App.css';

import LoginCard from "./login/LoginCard"
import LoginField from "./login/LoginField"

const Login = () => {
  const [isVisible, setIsVisible] = React.useState(false);
  const [isAccountVisible, setAccountIsVisible] = useState(false);
  const [selectedUser, setSelectedUser] = useState(null);

  const handleCardClick = (user) => {
    console.log("debug message");
    setSelectedUser(user);
    setAccountIsVisible(true);
  };

  const handleClose = () => {
    setAccountIsVisible(false);
  };


  const [loginCardsList, setLoginCardsList] = useState([
    { id: '1', imageSrc: "./media/img/locker2.svg", name: "Johnny"},
    { id: '2', imageSrc: "./media/img/locker2.svg", name: "Frank"},
    { id: '3', imageSrc: "./media/img/locker2.svg", name: "Max"}
  ]);

  const loginCards = loginCardsList.map((card) => (
      <LoginCard key={card.id} card={card} onClick={() => handleCardClick(card)}/>
  ));
  
  

  return (
    <>
    <div className="w-full items-center flex flex-col">
          <h1 className="mb-4 text-8xl font-extrabold leading-none tracking-tight text-gray-900 md:text-8xl lg:text-8xl dark:text-white">LOGIN</h1>
        </div>
        {!isAccountVisible ? (
        <div className='flex flex-row justify-center items-center flex-wrap mt-4'>
          {loginCards}
        </div>
      ) : (
        <LoginField account={selectedUser} onClose={handleClose} />
      )}
    </>
  );
};

export default Login;