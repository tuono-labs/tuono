import { execSync } from 'child_process'
import * as path from 'path'

const __dirname = import.meta.dirname

const appDir = path.join(__dirname, 'fixtures', 'base')

execSync('cargo build --config opt-level=0', { stdio: 'inherit' })
execSync('pnpm --ignore-workspace install', { cwd: appDir, stdio: 'inherit' })
