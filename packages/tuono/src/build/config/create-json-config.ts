import { promises as fs } from 'fs'
import * as path from 'path'

import type { InternalTuonoConfig } from '../types'

import {
  DOT_TUONO_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  SERVER_CONFIG_NAME,
} from '../constants'

const CONFIG_PATH = [
  DOT_TUONO_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  SERVER_CONFIG_NAME,
].join(path.sep)

/**
 * This function is used to remove the `vite` property from the config object.
 * This is needed because the `vite` property is not needed neither by the server nor the client.
 */
function removeViteProperties(
  config: InternalTuonoConfig,
): Omit<InternalTuonoConfig, 'vite'> {
  const newConfig = structuredClone(config)
  delete newConfig['vite']
  return newConfig
}

/**
 * This function creates a JSON config file that can be used by the server and
 * then shared to the client as prop.
 * The created file will be created at `.tuono/config/config.json`.
 *
 * The file needs to be a JSON in order to be easily read by the server written
 * in rust.
 */
export async function createJsonConfig(
  config: InternalTuonoConfig,
): Promise<void> {
  const jsonConfig = removeViteProperties(config)

  const fullPath = path.resolve(CONFIG_PATH)
  const jsonContent = JSON.stringify(jsonConfig)

  // No need to manage error state. Tuono CLI will manage it.
  await fs.writeFile(fullPath, jsonContent, 'utf-8')
}
