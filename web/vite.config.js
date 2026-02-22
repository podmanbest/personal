import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      // API port: sesuaikan dengan PORT di api/configs/.env (Windows: pakai 3000 jika 8080 diblokir)
      '/api': { target: process.env.VITE_API_PORT ? `http://localhost:${process.env.VITE_API_PORT}` : 'http://localhost:3000', changeOrigin: true },
      '/login': {
        target: process.env.VITE_API_PORT ? `http://localhost:${process.env.VITE_API_PORT}` : 'http://localhost:3000',
        changeOrigin: true,
        // Navigasi/refresh ke /login (GET, Accept: text/html) → serve SPA; hanya POST login yang di-proxy ke API.
        bypass(req) {
          if (req.method !== 'POST' && req.headers.accept?.includes('text/html')) {
            return '/index.html'
          }
        },
      },
      '/admin': {
        target: process.env.VITE_API_PORT ? `http://localhost:${process.env.VITE_API_PORT}` : 'http://localhost:3000',
        changeOrigin: true,
        // Saat refresh di /admin, browser minta dokumen (Accept: text/html) — jangan proxy,
        // biarkan Vite serve index.html agar SPA load; hanya request API (fetch/XHR) yang di-proxy.
        bypass(req) {
          if (req.headers.accept?.includes('text/html')) {
            return '/index.html'
          }
        },
      },
    },
  },
})
