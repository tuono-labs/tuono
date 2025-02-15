/// <reference types="vitest" />
/// <reference types="vite/client" />
import { defineConfig, mergeConfig } from 'vitest/config'
import { defineViteConfig } from 'vite-config'
import react from '@vitejs/plugin-react-swc'

const config = defineConfig({
  plugins: [react()],
})

export default mergeConfig(
  config,
  defineViteConfig({
    entry: [
      './src/index.ts',
      './src/build/index.ts',
      './src/config/index.ts',
      './src/ssr/index.ts',
      './src/hydration/index.tsx',
    ],
  }),
)
