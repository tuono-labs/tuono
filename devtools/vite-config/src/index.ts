import type { UserConfig } from 'vite'
import { preserveDirectives } from 'rollup-plugin-preserve-directives'
import { externalizeDeps } from 'vite-plugin-externalize-deps'
import tsconfigPaths from 'vite-tsconfig-paths'

interface Options {
  /** Entry file, e.g. `./src/index.ts` */
  entry: string | Array<string>
}

export function defineViteConfig(options: Options): UserConfig {
  return {
    build: {
      outDir: 'dist',
      minify: false,
      sourcemap: true,
      lib: {
        entry: options.entry,
        formats: ['es'],
        fileName: 'esm/[name]',
      },
      rollupOptions: {
        output: {
          preserveModules: true,
        },
      },
    },
    plugins: [externalizeDeps(), preserveDirectives(), tsconfigPaths()],
  }
}
