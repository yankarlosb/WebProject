import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/  (configuración de Vite)
export default defineConfig({
  plugins: [vue(), tailwindcss()],
  server: {
    port: 5173,
    proxy: {
      // Proxy para las llamadas a la API en desarrollo
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
      }
    }
  },
  build: {
    // Directorio de salida para producción
    outDir: 'dist',
    // Generar sourcemaps para debug
    sourcemap: true,
  }
})
