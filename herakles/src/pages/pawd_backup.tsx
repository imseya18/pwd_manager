import React, {useState, useEffect} from 'react';
import { NextUIProvider, Select, SelectItem , Input, Spacer, Card, CardFooter, Image, Button} from '@nextui-org/react';
import '../App.css';




const PasswordPolicy = () => {
	const [value, setValue] = React.useState("");
	const [isError, setIsError] = React.useState(false);
	const [errorMsg, setErrorMsg] = React.useState("");

	const [containsLowercase, setContainsLowercase] = React.useState(false);
	const [containsUppercase, setContainsUppercase] = React.useState(false);
	const [containsNumber, setContainsNumber] = React.useState(false);
	const [containsSpecialChar, setContainsSpecialChar] = React.useState(false);
	const [goodLength, setGoodLength] = React.useState(false);

	
	/*
		status:
			0: Empty
			1: Need lowercase
			2: Need uppercase
			3: Need number
			4: Need special char
			5: Length so short
			10: Good

	*/

	const hasLowerCase = (str: string) => {
		return (/[a-z]/.test(str));
	}

	const hasUpperCase = (str: string) => {
		return (/[A-Z]/.test(str));
	}

	const hasNumber = (str: string) => {
		return (/[0-9]/.test(str));
	}

	const hasSpecialChars = (str: string) => {
		const regex = /[\!"#$%&'()*+,\-./:;<=>?@\[\\\]^_`{|}~]/;
		return regex.test(str);
	}

	const isRespectPasswordPolicy = (str: string) => {

		if (str.length == 0)
			return 0;
		if (!hasLowerCase(str))
			return 1;
		if (!hasUpperCase(str))
			return 2;
		if (!hasNumber(str))
			return 3;
		if (!hasSpecialChars(str))
			return 4;
		if (str.length < 12)
			return 5;
		return 10;
	}

	const validatePassword = (str) => {
		setContainsLowercase(hasLowerCase(str));
		setContainsUppercase(hasUpperCase(str));
		setContainsNumber(hasNumber(str));
		setContainsSpecialChar(hasSpecialChars(str));
		setGoodLength(str.length >= 12);
	  };

	const onChangePassword = (e) => {
		setValue(e);

		validatePassword(e);

		if (isRespectPasswordPolicy(e) == 10)
		{
			setIsError(false);
			setErrorMsg("");
			return;
		}

		setIsError(true);

		let error_msg = "";

		if (e.length == 0)
		{
			setErrorMsg("Please enter a password");
			return ;
		}
		if (!hasLowerCase(e))
			error_msg += "• Needs to contain a lowercase\n";
		if (!hasUpperCase(e))
			error_msg += "• Needs to contain a uppercase\n";
		if (!hasNumber(e))
			error_msg += "• Needs to contain a number\n";
		if (!hasSpecialChars(e))
			error_msg +="• Needs to contain a special character\n";
		if (e.length < 12)
			error_msg += "• Too short\n";
		setErrorMsg(error_msg);
	}
  
  return (
    <div className="w-full flex flex-col gap-2 max-w-[240px]">
      <Input
        label="Password"
        placeholder="Enter your password"
        value={value}
        onValueChange={onChangePassword}
		isInvalid={isError}
      	errorMessage=""
      />
	  <div className='w-full flex flex-col gap-2 max-w-[240px]'>
		<p className={`text-tiny ${containsLowercase ? 'text-success' : 'text-danger'}`}>
			• Needs to contain a lowercase
		</p>
		<p className={`text-tiny ${containsUppercase ? 'text-success' : 'text-danger'}`}>
			• Needs to contain an uppercase
		</p>
		<p className={`text-tiny ${containsNumber ? 'text-success' : 'text-danger'}`}>
			• Needs to contain a number
		</p>
		<p className={`text-tiny ${containsSpecialChar ? 'text-success' : 'text-danger'}`}>
			• Needs to contain a special character
		</p>
		<p className={`text-tiny ${goodLength ? 'text-success' : 'text-danger'}`}>
			• Password is too short
		</p>
		</div>

    </div>
	
  );
};

export default PasswordPolicy;