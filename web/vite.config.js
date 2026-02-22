import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/api': { target: 'http://localhost:8080', changeOrigin: true },
      '/login': { target: 'http://localhost:8080', changeOrigin: true },
      '/status': { target: 'http://localhost:8080', changeOrigin: true },
      '/admin': { target: 'http://localhost:8080', changeOrigin: true },
      '/admin/skill-categories': { target: 'http://localhost:8080', changeOrigin: true },
      '/admin/skills': { target: 'http://localhost:8080', changeOrigin: true },
    },
  },
})
