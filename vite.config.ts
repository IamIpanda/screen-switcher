import { defineConfig } from 'vite'
import preact from '@preact/preset-vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [preact()],
  base: "./",
  server: {
    proxy: {
      "/set": "http://192.168.6.13:9999",
      "/reset": "http://192.168.6.13:9999"
    }
  }
})
