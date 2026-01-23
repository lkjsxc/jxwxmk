import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
    root: resolve(__dirname, 'src'),
    publicDir: resolve(__dirname, 'public'),
    build: {
        outDir: resolve(__dirname, '../../dist/frontend'),
        emptyOutDir: true,
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'src/main.ts'),
            },
        },
    },
    server: {
        port: 3000,
        strictPort: true,
    },
    preview: {
        port: 3000,
    },
});