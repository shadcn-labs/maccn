import { defineConfig } from 'vite';

// Builds the WASM demo into the docs site's public directory so the Next.js
// docs can embed each component in an iframe at /examples/?component=<name>.
export default defineConfig({
  base: '/examples/',
  build: {
    outDir: '../../../../../docs/public/examples',
    emptyOutDir: true,
  },
  server: {
    host: true,
  },
});
