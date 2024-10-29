import React, {useState, useEffect} from 'react';
import { NextUIProvider, Select, SelectItem , Input, Spacer, Card, CardFooter, Image, Button} from '@nextui-org/react';
import '../App.css';
import {EyeFilledIcon} from "../media/jsx/EyeFilledIcon";
import {EyeSlashFilledIcon} from "../media/jsx/EyeSlashFilledIcon";


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

	export const isRespectRegisterPassword = (str: string) => {

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

const RegisterPassword = ({
	value,
	setValue,
	Revalue,
	setReValue,
	isError,
	isReError,
	errorMsg}) => {

	const [containsLowercase, setContainsLowercase] = React.useState(false);
	const [containsUppercase, setContainsUppercase] = React.useState(false);
	const [containsNumber, setContainsNumber] = React.useState(false);
	const [containsSpecialChar, setContainsSpecialChar] = React.useState(false);
	const [goodLength, setGoodLength] = React.useState(false);

	const [progressBar, setProgressBar] = React.useState(0);
	const [progressBarColor, setProgressBarColor] = React.useState("");
	const [progressBarTitle, setProgressBarTitle] = React.useState("");

	const [isVisiblePwd, setIsVisiblePwd] = React.useState(false);
	const [isVisibleRePwd, setIsVisibleRePwd] = React.useState(false);

	const toggleVisibilityPwd = () => setIsVisiblePwd(!isVisiblePwd);
	const toggleVisibilityRePwd = () => setIsVisibleRePwd(!isVisibleRePwd);
	
	

	const validatePassword = (str) => {
		setContainsLowercase(hasLowerCase(str));
		setContainsUppercase(hasUpperCase(str));
		setContainsNumber(hasNumber(str));
		setContainsSpecialChar(hasSpecialChars(str));
		setGoodLength(str.length >= 12);
	  };

	

	const passwordProgressBar = (score) => {
		switch(score) {
		  case 0:
			setProgressBar(0);
			setProgressBarColor("transparent");
			setProgressBarTitle("No password entered");
			break;
		  case 1:
			setProgressBar(16);
			setProgressBarColor("#ff0000b5");
			setProgressBarTitle("Very Weak");
			break;
		  case 2:
			setProgressBar(32);
			setProgressBarColor("#ff6000b5");
			setProgressBarTitle("Weak");
			break;
		  case 3:
			setProgressBar(48);
			setProgressBarColor("#ffed00b5");
			setProgressBarTitle("Fair");
			break;
		  case 4:
			setProgressBar(64);
			setProgressBarColor("#a7ff00b5");
			setProgressBarTitle("Good");
			break;
		  case 5:
			setProgressBar(80);
			setProgressBarColor("#60ff00b5");
			setProgressBarTitle("Strong");
			break;
		  case 6:
			setProgressBar(100);
			setProgressBarColor("#31ff00b5");
			setProgressBarTitle("Very Strong");
			break;
		  default:
			setProgressBar(0);
			setProgressBarColor("transparent");
			setProgressBarTitle("Error: Invalid score");
		}
	  };

	const onChangeConfirmPassword = (e) => {
		setReValue(e);
	}

	const onChangePassword = (e) => {
		setValue(e);

		validatePassword(e);

		let score = 0;
		passwordProgressBar(score);

		if (e.length == 0)
		{
			setErrorMsg("Please enter a password");
			return ;
		}
		if (hasLowerCase(e))
			score += 1;
		if (hasUpperCase(e))
			score += 1;
		if (hasNumber(e))
			score += 1;
		if (hasSpecialChars(e))
			score += 1;
		if (e.length >= 12)
			score += 1;
		if (e.length >= 16)
			score += 1;
		passwordProgressBar(score);
		//setErrorMsg(error_msg);
	}
  
	

  return (
    <div className="w-full flex flex-col gap-2 max-w-[240px]" style={{marginTop:0}}>
      <Input
        label="Password"
        placeholder="Enter your password"
		endContent={
			<button className="focus:outline-none" type="button" onClick={toggleVisibilityPwd} aria-label="toggle password visibility">
			  {isVisiblePwd ? (
				<EyeSlashFilledIcon className="text-2xl text-default-400 pointer-events-none" />
			  ) : (
				<EyeFilledIcon className="text-2xl text-default-400 pointer-events-none" />
			  )}
			</button>
		  }
		type={isVisiblePwd ? "text" : "password"}
        value={value}
        onValueChange={onChangePassword}
		isInvalid={isError}
      />

	  <div className='w-full flex flex-col gap-2 max-w-[240px]'>

		<div className='w-full flex max-w-[240px] bg-default-100' style={{height:5, borderRadius:15}}>
		<div
		  className='passwordProgressBar'
		  style={{
			borderRadius: 15,
			background: `${progressBarColor}`,
			width: `${progressBar}%`
			}}></div>
		</div>
		<div className='w-full flex justify-center gap-2 max-w-[240px]'>
			<p className=''style={{color: `${progressBarColor}`}}>{progressBarTitle}</p>
		</div>
		

      {!containsLowercase && (
        <p className='text-tiny text-stone-500'>• Needs to contain a lowercase</p>
      )}
      {!containsUppercase && (
        <p className='text-tiny text-stone-500'>• Needs to contain an uppercase</p>
      )}
      {!containsNumber && (
        <p className='text-tiny text-stone-500'>• Needs to contain a number</p>
      )}
      {!containsSpecialChar && (
        <p className='text-tiny text-stone-500'>• Needs to contain a special character</p>
      )}
      {!goodLength && (
        <p className='text-tiny text-stone-500'>• Password is too short</p>
      )}
    </div>

	<div style={{marginTop:20}}>
		<Input
			label="Confirm Password"
			placeholder="Retype your password"
			endContent={
				<button className="focus:outline-none" type="button" onClick={toggleVisibilityRePwd} aria-label="toggle password visibility">
				{isVisibleRePwd ? (
					<EyeSlashFilledIcon className="text-2xl text-default-400 pointer-events-none" />
				) : (
					<EyeFilledIcon className="text-2xl text-default-400 pointer-events-none" />
				)}
				</button>
			}
			type={isVisibleRePwd ? "text" : "password"}
			value={Revalue}
			onValueChange={onChangeConfirmPassword}
			isInvalid={isReError}
			errorMessage={errorMsg}
		/>
		</div>
	</div>
	
  );
};

export default RegisterPassword;