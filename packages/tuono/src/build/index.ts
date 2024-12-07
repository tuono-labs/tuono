import type { InlineConfig } from 'vite'
import { build, createServer, mergeConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import ViteFsRouter from 'tuono-fs-router-vite-plugin'
import { LazyLoadingPlugin } from 'tuono-lazy-fn-vite-plugin'

import type { TuonoConfig } from '../config'

import { loadConfig, blockingAsync } from './utils'

const VITE_PORT = 3001

/**
 * From a given {@link TuonoConfig} return a `vite` "mergeable" {@link InlineConfig}
 * including all default tuono related options
 */
function createBaseViteConfigFromTuonoConfig(
  tuonoConfig: TuonoConfig,
): InlineConfig {
  const viteBaseConfig: InlineConfig = {
    root: '.tuono',
    logLevel: 'silent',
    publicDir: '../public',
    cacheDir: 'cache',
    envDir: '../',

    resolve: {
      alias: tuonoConfig.vite?.alias ?? {},
    },

    optimizeDeps: tuonoConfig.vite?.optimizeDeps,

    plugins: [
      ...(tuonoConfig.vite?.plugins ?? []),
      /** @todo todo remove this (https://github.com/tuono-labs/tuono/issues/183#issuecomment-2518360580)  */
      // @ts-expect-error: TS configuration issue.
      react({ include: /\.(jsx|js|mdx|md|tsx|ts)$/ }),
      ViteFsRouter(),
      LazyLoadingPlugin(),
    ],
  }

  // seems redundant but it's useful to log the value when debugging, until we have a logging infrastructure.
  return viteBaseConfig
}

const developmentSSRBundle = (): void => {
  blockingAsync(async () => {
    const config = await loadConfig()
    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTuonoConfig(config),
        {
          build: {
            ssr: true,
            minify: false,
            outDir: 'server',
            emptyOutDir: true,
            rollupOptions: {
              input: './.tuono/server-main.tsx',
              onLog() {
                /* Silence all logs */
              },
              output: {
                entryFileNames: 'dev-server.js',
                format: 'iife',
              },
            },
          },
          ssr: {
            target: 'webworker',
            noExternal: true,
          },
        },
      ),
    )
  })
}

const developmentCSRWatch = (): void => {
  console.info('developmentCSRWatch')
  blockingAsync(async () => {
    const config = await loadConfig()
    const server = await createServer(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTuonoConfig(config),
        {
          // Entry point for the development vite proxy
          base: '/vite-server/',

          server: {
            port: VITE_PORT,
            strictPort: true,
          },
          build: {
            manifest: true,
            emptyOutDir: true,
            rollupOptions: {
              input: './.tuono/client-main.tsx',
            },
          },
        },
      ),
    )
    await server.listen()
  })
}

const buildProd = (): void => {
  blockingAsync(async () => {
    const config = await loadConfig()

    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTuonoConfig(config),
        {
          build: {
            manifest: true,
            emptyOutDir: true,
            outDir: '../out/client',
            rollupOptions: {
              input: './.tuono/client-main.tsx',
            },
          },
        },
      ),
    )

    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTuonoConfig(config),
        {
          build: {
            ssr: true,
            minify: true,
            outDir: '../out/server',
            emptyOutDir: true,
            rollupOptions: {
              input: './.tuono/server-main.tsx',
              output: {
                entryFileNames: 'prod-server.js',
                format: 'iife',
              },
            },
          },
          ssr: {
            target: 'webworker',
            noExternal: true,
          },
        },
      ),
    )
  })
}

const buildConfig = (): void => {
  blockingAsync(async () => {
    await build({
      root: '.tuono',
      logLevel: 'silent',
      cacheDir: 'cache',
      envDir: '../',
      build: {
        ssr: true,
        outDir: 'config',
        emptyOutDir: true,
        rollupOptions: {
          input: './tuono.config.ts',
          output: {
            entryFileNames: 'config.mjs',
          },
        },
      },
    })
  })
}

export { buildProd, buildConfig, developmentCSRWatch, developmentSSRBundle }
