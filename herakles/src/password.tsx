import React, {useState, useEffect} from 'react';
import {Accordion, AccordionItem, Divider, Input, Textarea} from "@nextui-org/react";
import pwdDatasetGen from './pwdDatasetGen';
import {EyeFilledIcon} from "./media/jsx/EyeFilledIcon";
import {EyeSlashFilledIcon} from "./media/jsx/EyeSlashFilledIcon";
// import CopyIcon from './media/jsx/copy'
import CopyIcon from './media/img/copy.svg'
import "./App.css";

export interface Account {
  email: string;
  password: string;
  note: string;
  last_modif: number;
}

export interface Site {
  id: string;
  url: string;
  account_number: number;
  accounts: Account[];
}

const formatDate = (timestamp) => {
  const date = new Date(timestamp);
  const day = String(date.getDate()).padStart(2, '0');
  const month = String(date.getMonth() + 1).padStart(2, '0'); // Les mois commencent à 0
  const year = date.getFullYear();
  
  return `${day}/${month}/${year}`;
};

const Favicon = ({ url }) => {
    const domain = new URL(url).hostname;
    const faviconUrl = `https://api.faviconkit.com/${domain}/32`;
  
    return <img src={faviconUrl} width={30} alt="Favicon" />;
  };


const Password = () => {
  const [visiblePasswords, setVisiblePasswords] = useState({});
  const toggleVisibility = (index_account) => {
    setVisiblePasswords((prevState) => ({
      ...prevState,
      [index_account]: !prevState[index_account],
    }));
  };
  
  


    const [passwordSite, setPasswordSite] = useState([
        { id: '1', url:'https://chatgpt.com', account_number: 1},
        { id: '2', url:'https://dokca.com', account_number: 1},
        { id: '3', url:'https://youtube.com', account_number: 14},
        { id: '4', url:'https://reddit.com', account_number: 5},
        { id: '5', url:'https://stackoverflow.com', account_number: 2},
        { id: '6', url:'https://haeghjfgjherfhj.com', account_number: 1},
      ]);

    // const data = pwdDatasetGen();
    // console.log(JSON.stringify(data, null, 2));

    const dataset: Site[] = [
      {
        "id": "1",
        "url": "https://chatgpt.com",
        "account_number": 5,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "Vérifier les mises à jour",
            "last_modif": 1726154347109
          },
          {
            "email": "user2@gmail.com",
            "password": "password2",
            "note": "Vérifier les mises à jour",
            "last_modif": 1726509590325
          },
          {
            "email": "user3@gmail.com",
            "password": "password3",
            "note": "Ne pas partager",
            "last_modif": 1725803427362
          },
          {
            "email": "user4@gmail.com",
            "password": "password4",
            "note": "Mot de passe temporaire",
            "last_modif": 1726421313246
          },
          {
            "email": "user5@gmail.com",
            "password": "password5",
            "note": "Compte professionnel",
            "last_modif": 1726311138385
          }
        ]
      },
      {
        "id": "2",
        "url": "https://dokca.com",
        "account_number": 4,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "Vérifier les mises à jour",
            "last_modif": 1726073843729
          },
          {
            "email": "user2@gmail.com",
            "password": "password2",
            "note": "Note importante",
            "last_modif": 1725879634090
          },
          {
            "email": "user3@gmail.com",
            "password": "password3",
            "note": "Compte secondaire",
            "last_modif": 1726363939779
          },
          {
            "email": "user4@gmail.com",
            "password": "password4",
            "note": "Ne pas partager",
            "last_modif": 1725799971703
          }
        ]
      },
      {
        "id": "3",
        "url": "https://youtube.com",
        "account_number": 2,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "À vérifier plus tard",
            "last_modif": 1726038330550
          },
          {
            "email": "user2@gmail.com",
            "password": "password2",
            "note": "Vérifier les mises à jour",
            "last_modif": 1726548181856
          }
        ]
      },
      {
        "id": "4",
        "url": "https://reddit.com",
        "account_number": 1,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "Mot de passe temporaire",
            "last_modif": 1725849076551
          }
        ]
      },
      {
        "id": "5",
        "url": "https://stackoverflow.com",
        "account_number": 3,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "Compte secondaire",
            "last_modif": 1725952075527
          },
          {
            "email": "user2@gmail.com",
            "password": "password2",
            "note": "Compte personnel",
            "last_modif": 1725948798528
          },
          {
            "email": "user3@gmail.com",
            "password": "password3",
            "note": "Compte personnel",
            "last_modif": 1726411937175
          }
        ]
      },
      {
        "id": "6",
        "url": "https://haeghjfgjherfhj.com",
        "account_number": 5,
        "accounts": [
          {
            "email": "user1@gmail.com",
            "password": "password1",
            "note": "À vérifier plus tard",
            "last_modif": 1726594846624
          },
          {
            "email": "user2@gmail.com",
            "password": "password2",
            "note": "Compte secondaire",
            "last_modif": 1726638818011
          },
          {
            "email": "user3@gmail.com",
            "password": "password3",
            "note": "Ne pas partager",
            "last_modif": 1726071073062
          },
          {
            "email": "user4@gmail.com",
            "password": "password4",
            "note": "Ne pas partager",
            "last_modif": 1726668329609
          },
          {
            "email": "user5@gmail.com",
            "password": "password5",
            "note": "Compte principal",
            "last_modif": 1726304286835
          }
        ]
      }
    ]
    ;

    const defaultContent =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

      const passwordSiteList =  dataset.map((site, index) => (
        <AccordionItem
            key={site.id}
            aria-label={site.url.replace(/^https?:\/\//, '')}
            startContent={<Favicon url={site.url} />}
            subtitle={`${site.account_number} account${site.account_number > 1 ? 's' : ''}`}
            title={site.url.replace(/^https?:\/\//, '')}
          >
             {site.accounts.map((account, index_account) => (
                <div key={index_account}>
                    <Divider />
                    <div className='flex row max-w m-4 w-full'>
                      <div className='flex flex-col w-3/12'>
                        <Input 
                            type="email"
                            variant="flat"
                            label="Email"
                            labelPlacement="outside"
                            value={account.email}
                            endContent={
                              <div className='flex row'>
                                <button>
                                  <img src={CopyIcon} width={23} className="text-2xl text-default-400 pointer-events-none"></img>
                                </button>
                              </div>
                            }
                            className="w-full m-2" />
                        <Input
                            label="Password"
                            labelPlacement="outside"
                            variant="flat"
                            value={account.password}
                            placeholder="Enter your password"
                            endContent={
                              <div className='flex row'>
                                <button
                                  className="focus:outline-none"
                                  type="button"
                                  onClick={() => toggleVisibility(index_account)}
                                  aria-label="toggle password visibility"
                                >
                                  {visiblePasswords[index_account] ? (
                                    <EyeSlashFilledIcon className="text-2xl text-default-400 pointer-events-none" />
                                  ) : (
                                    <EyeFilledIcon className="text-2xl text-default-400 pointer-events-none" />
                                  )}
                                </button>
                                <button className='ml-2'>
                                  {/* <CopyIcon className="text-2xl text-default-400 pointer-events-none" /> */}
                                  <img src={CopyIcon} width={32} className="text-2xl text-default-400 pointer-events-none"></img>
                                </button>
                              </div>
                            }
                            type={visiblePasswords[index_account] ? "text" : "password"}
                            className="w-full m-2 pwd-top-margin"
                          />
                        </div>
                        <Textarea 
                          label="Note"
                          labelPlacement="outside"
                          minRows={5}
                          maxRows={5}
                          placeholder="No added note"
                          className="max-w-xs m-2 ml-10 mt-0"
                          value={account.note}
                        />
                        <div className='flex flex-col ml-5'>
                            <p>Last modification</p>
                            <p style={{color:'rgba(255, 255, 255, 0.45)'}}>{formatDate(account.last_modif)}</p>
                        </div>
                      </div>
                    {/* {index_account < site.accounts.length -1 ? <Divider /> : null} */}

                    
                </div>
                ))}
        </AccordionItem>
        ));

    return (
        <div className="flex gap-4 items-center flex-col w-full" style={{ padding: '20px', margin: 'auto' }}>
        <div className="w-full items-center flex flex-col">
          <h1 className="mb-4 text-8xl font-extrabold leading-none tracking-tight text-gray-900 md:text-8xl lg:text-8xl dark:text-white">PASSWORD</h1>
        </div>
        <Accordion variant="splitted">
            {passwordSiteList}
            </Accordion>
        </div>
      );
}
export default Password;