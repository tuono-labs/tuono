import type { UserConfig } from 'vite'
import { preserveDirectives } from 'rollup-plugin-preserve-directives'
import { externalizeDeps } from 'vite-plugin-externalize-deps'
import UnpluginIsolatedDecl from 'unplugin-isolated-decl/vite'

interface Options {
  /** Entry file, e.g. `./src/index.ts` */
  entry: string | Array<string>
}

export function defineViteConfig({ entry }: Options): UserConfig {
  const outDir = 'dist'

  return {
    build: {
      outDir,
      minify: false,
      sourcemap: true,
      lib: {
        entry,
        formats: ['es'],
        fileName: 'esm/[name]',
      },
      rollupOptions: {
        output: {
          preserveModules: true,
          entryFileNames: 'esm/[name].js',
        },
      },
    },
    plugins: [
      externalizeDeps(),
      preserveDirectives(),
      UnpluginIsolatedDecl({ transformer: 'oxc' }),
    ],
  }
}
