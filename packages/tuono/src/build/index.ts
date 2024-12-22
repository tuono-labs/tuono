import type { InlineConfig, Plugin } from 'vite'
import { build, createServer, mergeConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import inject from '@rollup/plugin-inject'
import ViteFsRouter from 'tuono-fs-router-vite-plugin'
import { LazyLoadingPlugin } from 'tuono-lazy-fn-vite-plugin'

import type { TuonoConfig } from '../config'

import { loadConfig, blockingAsync } from './utils'

const VITE_PORT = 3001
const VITE_SSR_PLUGINS: Array<Plugin> = [
  {
    enforce: 'post',
    ...inject({
      ReadableStream: ['web-streams-polyfill', 'ReadableStream'],

      /**
       * Added to support `react@19`
       * @see https://github.com/tuono-labs/tuono/issues/218
       */
      MessageChannel: ['tuono/ssr', 'MessageChannelPolyfill'],
      MessageEvent: ['tuono/ssr', 'MessageEventPolyfill'],
      Event: ['tuono/ssr', 'EventPolyfill'],
    }),
  },
]

/**
 * From a given {@link TuonoConfig} return a `vite` "mergeable" {@link InlineConfig}
 * including all default tuono related options
 */
function createBaseViteConfigFromTuonoConfig(
  tuonoConfig: TuonoConfig,
): InlineConfig {
  /**
   * @warning Keep in sync with {@link LazyLoadingPlugin} tests:
   * packages/lazy-fn-vite-plugin/tests/transpileSource.test.ts
   */
  const pluginFilesInclude = /\.(jsx|js|mdx|md|tsx|ts)$/

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

      /**
       * even if `include` is not a valid option for this
       * plugin, we have to use it.
       * If not specified, when running `tuono dev`, the mdx
       * won't be compiled include any style in the page and it might
       * seem broken.
       */
      // @ts-expect-error see above comment
      react({ include: pluginFilesInclude }),

      ViteFsRouter(),
      LazyLoadingPlugin({ include: pluginFilesInclude }),
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
          plugins: VITE_SSR_PLUGINS,
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
          plugins: VITE_SSR_PLUGINS,
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
