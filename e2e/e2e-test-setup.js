import { execSync } from 'child_process'
import * as path from 'path'

const __dirname = import.meta.dirname

const appDir = path.join(__dirname, 'test-app')

execSync('cargo build --config opt-level=0', { stdio: 'inherit' })

const isWindows = process.platform === 'win32'
const tuonoBuildDir = path.join(
  __dirname,
  '..',
  'target',
  'debug',
  isWindows ? 'tuono.exe' : 'tuono',
)

execSync(`${tuonoBuildDir} new test-app --template tuono-tutorial`, {
  cwd: __dirname,
  stdio: 'inherit',
})

execSync('npm install', { cwd: appDir, stdio: 'inherit' })
