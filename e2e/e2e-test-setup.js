import { execSync } from 'child_process'
import path from 'path'

const __dirname = import.meta.dirname
const rootDir = path.join(__dirname, '..')

execSync('cargo build --config opt-level=0', { cwd: rootDir, stdio: 'inherit' })
execSync('turbo build', { stdio: 'inherit' })
