import { defineConfig } from 'vite';
import { resolve } from 'path';
import vue from '@vitejs/plugin-vue';

const assetsRoot = resolve(__dirname, 'assets');
const webRoot = resolve(assetsRoot, 'web');

export default defineConfig({
  plugins: [vue()],
  root: webRoot,
  base: './',
  build: {
    outDir: resolve(__dirname, 'dist/DSL-KeyPad/assets'),
    emptyOutDir: true,
    minify: false,
    rollupOptions: {
      input: {
        app: resolve(webRoot, 'index.html'),
      },
      output: {
        entryFileNames: '[name].js',
        chunkFileNames: '[name].js',
        assetFileNames: '[name].[ext]',
      },
    },
  },
  resolve: {
    alias: {
      '@assets': assetsRoot,
      '@shared': resolve(webRoot, 'shared'),
      '@styles': resolve(webRoot, 'styles'),
      '@windows': resolve(webRoot, 'windows'),
    },
  },
  server: {
    port: 3000,
    strictPort: true,
    open: '/index',
    cors: true,
    fs: { strict: false },
    hmr: { protocol: 'ws', host: 'localhost' },
  },
});
