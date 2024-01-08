/** @type {import('vite').UserConfig} */
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte()
  ],
  server : {
    proxy : {
      "/api" : {
        target : "http://localhost:5000",
        changeOrigin: true,
      }
    }
  },
  build: {
    rollupOptions: {
      input: {
        main: "index.html",
        admin: "admin.html",
      }
    }
  }
})
