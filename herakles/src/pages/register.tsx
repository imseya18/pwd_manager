import React, {useState, useEffect} from 'react';
import { NextUIProvider, Select, SelectItem , Input, Spacer, Card, CardFooter, Image, Button} from '@nextui-org/react';
import '../App.css';
import RegisterPassword from './RegisterPassword';
import { isRespectRegisterPassword } from './RegisterPassword';



const PasswordPolicy = () => {
	const [value, setValue] = React.useState("");
	const [Revalue, setReValue] = React.useState("");
	const [isError, setIsError] = React.useState(false);
	const [isReError, setIsReError] = React.useState(false);
	const [errorMsg, setErrorMsg] = React.useState("");

	const [pseudo, setPseudo] = React.useState("");
	const [isErrorPseudo, setIsErrorPseudo] = React.useState(false);
	const [pseudoErrorMsg, setPseudoErrorMsg] = React.useState("");

	const [email, setEmail] = React.useState("");
	const [isErrorEmail, setIsErrorEmail] = React.useState(false);
	const [emailErrorMsg, setEmailErrorMsg] = React.useState("");
	
	const isValidPseudo = (pseudo) => {
		const regex = /^[a-zA-Z0-9_-]{3,16}$/;
		return regex.test(pseudo);
	  }

	const isUniquePseudo = (pseudo) => {
		if (pseudo.length == 0)
			return (1);
		if (!isValidPseudo(pseudo))
			return (2);
		//if (pseudoAlreadyUsed(pseudo))
		//	return (3);
		return (10)
	}
  
	const isValidEmail = (email) => {
		let re = /\S+@\S+\.\S+/;

		if (email.length == 0)
			return (1);
  		if (!re.test(email))
			return (2);
		return (10);
	}

	const onChangePseudo = (pseudo) => {
		setPseudo(pseudo);
		setIsErrorPseudo(false);
		setPseudoErrorMsg("");
	}

	const onChangeEmail = (email) => {
		setEmail(email);
		setIsErrorEmail(false);
		setEmailErrorMsg("");
	}

	const onChangePassword = (pwd) => {
		setValue(pwd);
		setIsError(false);
		setIsReError(false);
		setErrorMsg("");
	}

	const onChangeRePassword = (pwd) => {
		setReValue(pwd);
		setIsError(false);
		setIsReError(false);
		setErrorMsg("");
	}

	const register = () => {
		let registerError = false;

		if (isUniquePseudo(pseudo) != 10)
		{
			let statuscode = isUniquePseudo(pseudo);

			setIsErrorPseudo(true);
			if (statuscode == 1)
				setPseudoErrorMsg("Empty field");
			else if (statuscode == 2)
				setPseudoErrorMsg("Invalid pseudo");
			else if (statuscode == 3)
				setPseudoErrorMsg("Pseudo already Used");
			registerError = true;
		}
		if (isValidEmail(email) != 10)
		{
			let statuscode = isValidEmail(pseudo);

			setIsErrorEmail(true);
			if (statuscode == 1)
				setEmailErrorMsg("Empty field");
			else if (statuscode == 2)
				setEmailErrorMsg("Invalid email");
			registerError = true;
		}
		if (isRespectRegisterPassword(value) != 10)
		{
			setIsError(true);
			registerError = true;
		}
		if (value != Revalue)
		{
			registerError = true;
			setIsError(true);
			setIsReError(true);
			setErrorMsg("Not same password");
		}
	}

  return (
    <div className="w-full flex flex-col gap-2 items-center">
		<div className='w-full flex flex-col gap-2 max-w-[240px]'>
		<div>
			<Input
				label="Pseudo"
				placeholder="Enter your pseudo"
				value={pseudo}
				onValueChange={onChangePseudo}
				isInvalid={isErrorPseudo}
				errorMessage={pseudoErrorMsg}
			/>
		</div>
		<div style={{marginTop:0}}>
			<Input
				label="Email"
				placeholder="Enter your email"
				value={email}
				onValueChange={onChangeEmail}
				isInvalid={isErrorEmail}
				errorMessage={emailErrorMsg}
			/>
		</div>
		<RegisterPassword
			value={value}
			setValue={onChangePassword}
			Revalue={Revalue}
			setReValue={onChangeRePassword}
			isError={isError}
			isReError={isReError}
			errorMsg={errorMsg}
	
		/>
		<Button color="primary" onPress={register} style={{marginTop:50}}>Register</Button>
		</div>
	</div>
	
  );
};

export default PasswordPolicy;