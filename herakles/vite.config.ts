import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  css: {
    postcss: './postcss.config.js',
  },

  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: [
        "**/src-tauri/**",   // Ignorer le dossier src-tauri
        "**/data/**",        // Ignorer le dossier data où la base de données est déplacée
        "**/*.db",           // Ignorer tous les fichiers .db, si nécessaire
        "**/logs/**",
        "**/stored/**",      // Ignorer les dossiers de logs, si applicable
      ]
    },
  },
}));
