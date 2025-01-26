import type { TuonoConfig } from 'tuono/config'
import tailwindcss from '@tailwindcss/vite'

const config: TuonoConfig = {
  vite: {
    plugins: [tailwindcss()],
  },
}

export default config
