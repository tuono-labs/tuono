import fs from 'fs/promises'
import path from 'path'

import type { InternalTuonoConfig } from '../types'

import {
  DOT_TUONO_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  SERVER_CONFIG_NAME,
} from '../constants'

const CONFIG_PATH = path.join(
  DOT_TUONO_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  SERVER_CONFIG_NAME,
)
/**
 * This function is used to remove the `vite` property from the config object.
 * The `vite` property is only used at build time, so it is not needed by either the server or the client.
 */
function removeViteProperties(
  config: InternalTuonoConfig,
): Omit<InternalTuonoConfig, 'vite'> {
  const newConfig = structuredClone(config)
  delete newConfig['vite']
  return newConfig
}

/**
 * This function creates a JSON config file for the server,
 * that will be shared with the client as a prop.
 * The file will be saved at`.tuono/config/config.json`.
 *
 * The file is in JSON format to ensure it's easily read by the server,
 * which is written in Rust.
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
