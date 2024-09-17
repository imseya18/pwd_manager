// main.tsx ou index.tsx
import { NextUIProvider } from '@nextui-org/react';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from "./App";
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NextUIProvider>
  <main className="dark text-forgground bg-background h-screen">
      <App />
  </main>
    </NextUIProvider>
  </React.StrictMode>
);
