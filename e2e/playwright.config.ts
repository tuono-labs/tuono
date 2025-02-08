import * as path from 'path'

import { fileURLToPath } from 'url'

import { defineConfig } from '@playwright/test'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const tuonoDir = path.join(__dirname, '../', 'target', 'release', 'tuono')
const setupScript = path.join(__dirname, 'e2e-test-setup.js')

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  webServer: {
    command: `node ${setupScript} && cd test-app && ${tuonoDir} dev`,
    port: 3000,
    timeout: 420 * 1000,
    stdout: 'pipe',
    reuseExistingServer: false,
  },
  use: {
    baseURL: 'http://localhost:3000',
  },
})
