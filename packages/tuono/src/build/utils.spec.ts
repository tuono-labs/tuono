import path from 'node:path'

import { describe, expect, it, vi } from 'vitest'

import type { TuonoConfig } from '../config'

import { loadConfig, normalizeConfig } from './utils'

const PROCESS_CWD_MOCK = 'PROCESS_CWD_MOCK'

vi.spyOn(process, 'cwd').mockReturnValue(PROCESS_CWD_MOCK)

describe('loadConfig', () => {
  it('should error if the config does not exist', async () => {
    const consoleErrorSpy = vi
      .spyOn(console, 'error')
      .mockImplementation(() => undefined)
    await loadConfig()

    expect(consoleErrorSpy).toHaveBeenCalledTimes(2)
  })
})

describe('normalizeConfig - vite', () => {
  it('should empty base config if empty config is provided', () => {
    const config: TuonoConfig = {}

    expect(normalizeConfig(config)).toStrictEqual({
      vite: { alias: undefined, optimizeDeps: undefined, plugins: [] },
    })
  })

  it('should return an empty config if invalid values are provided', () => {
    // @ts-expect-error testing invalid config
    expect(normalizeConfig({ invalid: true })).toStrictEqual({
      vite: { alias: undefined, optimizeDeps: undefined, plugins: [] },
    })
  })
})

describe('normalizeConfig - vite - alias', () => {
  it('should not modify alias pointing to packages', () => {
    const libraryName = '@tabler/icons-react'
    const libraryAlias = '@tabler/icons-react/dist/esm/icons/index.mjs'
    const config: TuonoConfig = {
      vite: { alias: { [libraryName]: libraryAlias } },
    }

    expect(normalizeConfig(config)).toStrictEqual(
      expect.objectContaining({
        vite: expect.objectContaining({
          alias: {
            '@tabler/icons-react':
              '@tabler/icons-react/dist/esm/icons/index.mjs',
          },
        }) as unknown,
      }),
    )
  })

  it('should transform relative paths to absolute path relative to process.cwd()', () => {
    const config: TuonoConfig = {
      vite: { alias: { '@': './src', '@no-prefix': 'src' } },
    }

    expect(normalizeConfig(config)).toStrictEqual(
      expect.objectContaining({
        vite: expect.objectContaining({
          alias: {
            '@': path.join(PROCESS_CWD_MOCK, 'src'),
            '@no-prefix': path.join(PROCESS_CWD_MOCK, 'src'),
          },
        }) as unknown,
      }),
    )
  })

  it('should not transform alias with absolute path', () => {
    const config: TuonoConfig = {
      vite: { alias: { '@1': '/src/pippo', '@2': 'file://pluto' } },
    }

    expect(normalizeConfig(config)).toStrictEqual(
      expect.objectContaining({
        vite: expect.objectContaining({
          alias: {
            '@1': '/src/pippo',
            '@2': 'file://pluto',
          },
        }) as unknown,
      }),
    )
  })

  it('should apply previous behavior when using alias as list', () => {
    const config: TuonoConfig = {
      vite: {
        alias: [
          { find: '1', replacement: '@tabler/icons-react-fun' },
          { find: '2', replacement: './src' },
          { find: '3', replacement: 'file://pluto' },
        ],
      },
    }

    expect(normalizeConfig(config)).toStrictEqual(
      expect.objectContaining({
        vite: expect.objectContaining({
          alias: [
            {
              find: '1',
              replacement: '@tabler/icons-react-fun',
            },
            {
              find: '2',
              replacement: path.join(PROCESS_CWD_MOCK, 'src'),
            },
            {
              find: '3',
              replacement: 'file://pluto',
            },
          ],
        }) as unknown,
      }),
    )
  })
})
