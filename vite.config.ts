import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
  root: 'web',

  build: {
		outDir: '../web_dev',
		target: 'chrome112',
		cssTarget: 'chrome112',
		minify: false,
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'web/src/dummy.html'),
      },
    },
  },

  server: {
    port: 3000,
    open: '/src/dummy.html',
  },
});
