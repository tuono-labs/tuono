import type { TuonoConfig } from 'tuono/config'
import mdx from '@mdx-js/rollup'

const config: TuonoConfig = {
  vite: {
    optimizeDeps: {
      exclude: ['@mdx-js/react'],
    },
    plugins: [
      { enforce: 'pre', ...mdx({ providerImportSource: '@mdx-js/react' }) },
    ],
  },
}

export default config
