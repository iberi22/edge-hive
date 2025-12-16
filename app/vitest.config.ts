import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

export default defineConfig({
   plugins: [
      svelte({ 
         hot: !process.env.VITEST,
         compilerOptions: {
            dev: true,
            hydratable: true
         },
         onwarn: (warning, handler) => {
            // Suppress warnings during tests
            if (warning.code === 'css-unused-selector') return;
            handler(warning);
         }
      })
   ],
   test: {
      globals: true,
      environment: 'happy-dom',
      setupFiles: ['./src/test/setup.ts'],
      include: ['src/**/*.{test,spec}.{js,ts}'],
      server: {
         deps: {
            inline: ['svelte']
         }
      },
      coverage: {
         provider: 'v8',
         reporter: ['text', 'html', 'lcov'],
         exclude: [
            'node_modules/',
            'src/test/',
            '**/*.d.ts',
            '**/*.config.*',
            '**/mockData.ts',
         ],
      },
   },
   resolve: {
      alias: {
         '@': path.resolve(__dirname, './src'),
         '$lib': path.resolve(__dirname, './src/lib'),
      },
   },
});
