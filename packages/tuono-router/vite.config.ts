/// <reference types="vitest" />
/// <reference types="vite/client" />
import { defineConfig, mergeConfig } from 'vitest/config'
import { defineViteConfig } from 'vite-config'
import react from '@vitejs/plugin-react-swc'

export default mergeConfig(
  defineConfig({
    plugins: [react()],
    test: {
      name: 'tuono-router',
      environment: 'happy-dom',
      globals: true,
    },
  }),
  defineViteConfig({
    entry: './src/index.ts',
    srcDir: './src',
  }),
)
