import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    typescript(),
    svelte({
      preprocess: sveltePreprocess(),
    })
  ],
})
