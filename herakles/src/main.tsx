// main.tsx ou index.tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import Page from './page';
import { NextUIProvider, Image } from '@nextui-org/react';

import logo from './media/img/logo_transparent.svg';


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NextUIProvider>
      <main className='dark text-foreground bg-background'>
          <div className="relative flex flex-col h-screen">
            <div className="w-full flex flex-row items-center py-5 px-5">
              <Image
                width={50}
                alt="Home"
                src={logo}
              />
              <h2 className="text-2xl leading-none tracking-tight text-gray-900 md:text-2xl lg:text-2xl dark:text-white pl-1" style={{ fontWeight: '500' }}>Herakles</h2>
            </div>
          
            <div className="container mx-auto pt-2 px-6 flex-grow">
              <Page />
            </div>
          </div>
          </main>
    </NextUIProvider>
  </React.StrictMode>
);
