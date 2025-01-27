/// <reference types="vitest" />
/// <reference types="vite/client" />
import { defineConfig, mergeConfig } from 'vitest/config'
import { tanstackBuildConfig } from '@tanstack/config/build'
import react from '@vitejs/plugin-react-swc'

const config = defineConfig({
  plugins: [react()],
  test: {
    name: 'tuono-router',
    environment: 'happy-dom',
    globals: true,
  },
})

export default mergeConfig(
  config,
  tanstackBuildConfig({
    entry: './src/index.ts',
    srcDir: './src',
  }),
)
