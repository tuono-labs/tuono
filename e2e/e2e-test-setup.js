import { execSync } from 'child_process'

execSync('cargo build --config opt-level=0', { stdio: 'inherit' })
execSync('turbo build', { stdio: 'inherit' })
