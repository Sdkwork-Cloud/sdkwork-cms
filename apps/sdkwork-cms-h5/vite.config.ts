import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, __dirname, '');
  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
      plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3001,
    host: '0.0.0.0',
    proxy: {
      '/app/v3/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
