import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    resolve: {
        preserveSymlinks: true
    },
    plugins: [sveltekit()]
});
