/** @type {import('tailwindcss').Config} */
const { nextui } = require('@nextui-org/react');

export default {
  content: ['./index.html',
    './src/**/*.{js,ts,jsx,tsx}',
    // Ajoutez ce chemin pour inclure les composants de Next UI
    './node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}',],
  theme: {
    extend: {},
  },
  darkMode:'class',
  plugins: [nextui(),],
}

