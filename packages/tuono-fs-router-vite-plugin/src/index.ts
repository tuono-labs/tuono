import { normalize } from 'path'

import type { Plugin } from 'vite'

import { routeGenerator } from './generator'

const ROUTES_DIRECTORY_PATH = './src/routes'

let lock = false

export default function RouterGenerator(): Plugin {
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

  return {
    name: 'vite-plugin-tuono-fs-router',
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
  }
}
