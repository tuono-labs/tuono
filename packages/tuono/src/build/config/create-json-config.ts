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
  /**
   * Using {@link structuredClone} cause the following errors based on runtime env:
   * ```text
   * node
   * DOMException [DataCloneError]: configureServer(s){a.push(s)} could not be cloned.
   *
   * vitest
   * DataCloneError: (id) => id === runtimePublicPath ? id : void 0 could not be cloned.
   * ```
   * when vite plugins are passed inside to the config.
   *
   * Since the purpose of this function is to remove the vite object
   * we are going to use destructing rather than {@link structuredClone} and `delete`
   *
   * @see https://github.com/tuono-labs/tuono/issues/414
   */
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { vite, ...configRest } = config
  return structuredClone(configRest)
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
