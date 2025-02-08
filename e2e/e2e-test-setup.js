import { execSync } from 'child_process'
import * as path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const appDir = path.join(__dirname, 'test-app')

execSync('cargo build --release', { stdio: 'inherit' })

const isWindows = process.platform === 'win32'
const tuonoBuildDir = path.join(
  __dirname,
  '..',
  'target',
  'release',
  isWindows ? 'tuono.exe' : 'tuono',
)

execSync(`${tuonoBuildDir} new test-app --template tuono-tutorial`, {
  cwd: __dirname,
  stdio: 'inherit',
})

execSync('npm install', { cwd: appDir, stdio: 'inherit' })
