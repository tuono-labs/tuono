import type { UserConfig } from 'vite'
import { preserveDirectives } from 'rollup-plugin-preserve-directives'
import { externalizeDeps } from 'vite-plugin-externalize-deps'
import tsconfigPaths from 'vite-tsconfig-paths'
import dts from 'vite-plugin-dts'

interface Options {
  /** Entry file, e.g. `./src/index.ts` */
  entry: string | Array<string>

  /** Source directory used for type generation, e.g. `./src` */
  srcDir: string
}

export function defineViteConfig(options: Options): UserConfig {
  const outDir = 'dist'
  return {
    build: {
      outDir,
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
    plugins: [
      externalizeDeps(),
      preserveDirectives(),
      tsconfigPaths(),
      dts({
        outDir: `${outDir}/esm`,
        entryRoot: options.srcDir,
        include: options.srcDir,
      }),
    ],
  }
}
