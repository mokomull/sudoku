import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const __dirname = dirname(fileURLToPath(import.meta.url))

export default defineConfig({
  plugins: [
    react(),
    wasm(),
    topLevelAwait()
  ],
  build: {
    rollupOptions: {
      input: [
        resolve(__dirname, "index.html"),
        resolve(__dirname, "wtf.html"),
      ]
    }
  },
});
