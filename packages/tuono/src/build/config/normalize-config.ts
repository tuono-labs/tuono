import path from 'node:path'

import type { AliasOptions } from 'vite'

import type { TuonoConfig } from '../../config'

import type { InternalTuonoConfig } from '../types'

import { DOT_TUONO_FOLDER_NAME, CONFIG_FOLDER_NAME } from '../constants'

/**
 *  Normalize vite alias option:
 * - If the path starts with `src` folder, transform it to absolute, prepending the tuono root folder
 * - If the path is absolute, remove the ".tuono/config/" path from it
 * - Otherwise leave the path untouched
 */
const normalizeAliasPath = (aliasPath: string): string => {
  if (aliasPath.startsWith('./src') || aliasPath.startsWith('src')) {
    return path.join(process.cwd(), aliasPath)
  }

  if (path.isAbsolute(aliasPath)) {
    return aliasPath.replace(
      path.join(DOT_TUONO_FOLDER_NAME, CONFIG_FOLDER_NAME),
      '',
    )
  }

  return aliasPath
}

/**
 * From a given vite aliasOptions apply {@link normalizeAliasPath} for each alias.
 *
 * The config is bundled by `vite` and emitted inside {@link DOT_TUONO_FOLDER_NAME}/{@link CONFIG_FOLDER_NAME}.
 * According to this, we have to ensure that the aliases provided by the user are updated to refer to the right folders.
 *
 * @see https://github.com/Valerioageno/tuono/pull/153#issuecomment-2508142877
 */
const normalizeViteAlias = (alias?: AliasOptions): AliasOptions | undefined => {
  if (!alias) return

  if (Array.isArray(alias)) {
    return (alias as Extract<AliasOptions, ReadonlyArray<unknown>>).map(
      ({ replacement, ...userAliasDefinition }) => ({
        ...userAliasDefinition,
        replacement: normalizeAliasPath(replacement),
      }),
    )
  }

  if (typeof alias === 'object') {
    const normalizedAlias: AliasOptions = {}
    for (const [key, value] of Object.entries(alias)) {
      normalizedAlias[key] = normalizeAliasPath(value as string)
    }
    return normalizedAlias
  }

  return alias
}

/**
 * Wrapper function to normalize the tuono.config.ts file
 *
 * @warning Exported for unit test.
 *          There is no easy way to mock the module export and change it in every test
 *          and also testing the error
 */
export const normalizeConfig = (config: TuonoConfig): InternalTuonoConfig => {
  return {
    server: {
      host: config.server?.host ?? 'localhost',
      origin: config.server?.origin ?? 'http://localhost:3000',
      port: config.server?.port ?? 3000,
    },
    vite: {
      alias: normalizeViteAlias(config.vite?.alias),
      css: config.vite?.css,
      optimizeDeps: config.vite?.optimizeDeps,
      plugins: config.vite?.plugins ?? [],
    },
  }
}
