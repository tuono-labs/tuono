import path from 'node:path'
import { pathToFileURL } from 'node:url'

import type { TuonoConfig } from '../../config'

import type { InternalTuonoConfig } from '../types'

import {
  DOT_TUONO_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  CONFIG_FILE_NAME,
} from '../constants'

import { normalizeConfig } from './normalize-config'

export const loadConfig = async (): Promise<InternalTuonoConfig> => {
  try {
    const configFile = (await import(
      pathToFileURL(
        path.join(
          process.cwd(),
          DOT_TUONO_FOLDER_NAME,
          CONFIG_FOLDER_NAME,
          CONFIG_FILE_NAME,
        ),
      ).href
    )) as { default: TuonoConfig }

    return normalizeConfig(configFile.default)
  } catch (err) {
    console.error('Failed to load tuono.config.ts')
    throw err as Error
  }
}
