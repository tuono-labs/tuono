/// <reference types="vitest" />
/// <reference types="vite/client" />
import { defineConfig, mergeConfig } from 'vitest/config'
import { defineViteConfig } from 'vite-config'
import react from '@vitejs/plugin-react-swc'

export default mergeConfig(
  defineConfig({
    plugins: [react()],
  }),
  defineViteConfig({
    entry: [
      './src/index.ts',
      './src/build/index.ts',
      './src/config/index.ts',
      './src/build-client/index.ts',
      './src/ssr/index.ts',
      './src/hydration/index.tsx',
    ],
  }),
)
