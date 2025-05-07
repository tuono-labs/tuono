import { normalize } from 'node:path'

import type { Plugin, ViteDevServer } from 'vite'

import { routeGenerator } from './fs-routing/generator'
import { getStylesForComponentId, isCssModulesFile } from './styles'

const CRITICAL_CSS_PATH = '/vite-server/tuono_internal__critical_css'

const ROUTES_DIRECTORY_PATH = './src/routes'

let lock = false

export function TuonoReactPlugin(): Plugin {
  const generate = async (): Promise<void> => {
    if (lock) return
    lock = true

    try {
      await routeGenerator()
    } catch (err) {
      console.error(err)
    } finally {
      lock = false
    }
  }

  const handleFile = async (file: string): Promise<void> => {
    const filePath = normalize(file)

    if (filePath.startsWith(ROUTES_DIRECTORY_PATH)) {
      await generate()
    }
  }

  const cssModulesManifest: Record<string, string> = {}

  return {
    name: 'vite-plugin-tuono-react',
    configResolved: async (): Promise<void> => {
      await generate()
    },
    watchChange: async (
      file: string,
      context: { event: string },
    ): Promise<void> => {
      if (['create', 'update', 'delete'].includes(context.event)) {
        await handleFile(file)
      }
    },
    transform: (code, id): void => {
      if (isCssModulesFile(id)) {
        cssModulesManifest[id] = code
      }
    },
    configureServer: (server: ViteDevServer): void => {
      // Using middlewares in order to take advantage of async requests out of
      // the box
      // eslint-disable-next-line @typescript-eslint/no-misused-promises
      server.middlewares.use(async (req, res, next): Promise<void> => {
        const url = new URL(req.url || '', `http://${req.headers.host || ''}`)

        // Give the request handler access to the critical CSS in dev to avoid a
        // flash of unstyled content since Vite injects CSS file contents via JS
        if (url.pathname === CRITICAL_CSS_PATH) {
          const componentId = url.searchParams.get('componentId')
          const css = await getStylesForComponentId(
            server,
            componentId,
            cssModulesManifest,
          )

          res.writeHead(200, { 'Content-Type': 'text/css' })
          res.end(css)
          return
        }
        next()
      })
    },
  }
}
