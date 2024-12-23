import fs from 'node:fs/promises'
import os from 'node:os'

import { it, expect, describe } from 'vitest'
import type { Plugin } from 'vite'

import { LazyLoadingPlugin } from '../src'

type ViteTransformHandler = Exclude<
  Plugin['transform'],
  // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
  Function | undefined
>['handler']

// Create a type-safe transform method
function getTransform(): (...args: Parameters<ViteTransformHandler>) => string {
  /** @warning Keep in sync with {@link createBaseViteConfigFromTuonoConfig} */
  const pluginFilesInclude = /\.(jsx|js|mdx|md|tsx|ts)$/

  return LazyLoadingPlugin({ include: pluginFilesInclude }).transform as never
}

describe('"dynamic" sources', async () => {
  const folderNames = await fs.readdir(`${process.cwd()}/tests/sources`)

  describe.each(folderNames)('%s', async (folderName) => {
    const testDirPath = `${process.cwd()}/tests/sources/${folderName}`

    const sourceFilePath = `${testDirPath}/source.tsx`

    const sourceRaw = await fs.readFile(sourceFilePath, 'utf-8')
    /**
     * When adding `packages/lazy-fn-vite-plugin/tests/sources/dynamic-only` only
     * the test involving that fixture were broken on Windows... but not the one in the other fixtures:
     * - packages/lazy-fn-vite-plugin/tests/sources/vanilla
     * - packages/lazy-fn-vite-plugin/tests/sources/external-dynamic
     *
     * Awkwardly this doesn't happen on  `packages/fs-router-vite-plugin/tests/generator.spec.ts`
     *
     * Too much pain and sadness to investigate this right now.
     * Might worth creating an utility function in the future if this happens again
     */
    const source = sourceRaw.replace(new RegExp(os.EOL, 'g'), '\n')

    it('should generate file for client', async () => {
      const pluginTransform = getTransform()
      const clientBundle = pluginTransform(source, sourceFilePath)

      const expectedClientSrc = `${testDirPath}/client.expected.tsx`

      await expect(clientBundle).toMatchFileSnapshot(
        expectedClientSrc,
        `${testDirPath} client build should be equal to ${expectedClientSrc}`,
      )
    })

    it('should generate file for server', async () => {
      const pluginTransform = getTransform()
      const serverBundle = pluginTransform(source, sourceFilePath, {
        ssr: true,
      })

      const expectedServerSrc = `${testDirPath}/server.expected.tsx`

      await expect(serverBundle).toMatchFileSnapshot(
        expectedServerSrc,
        `${testDirPath} server build should be equal to ${expectedServerSrc}`,
      )
    })
  })
})

describe('not valid file should not be processed', () => {
  const notValidFiles: Array<string> = [
    'src/styles/module.css',
    'src/pages/file-without-ext',
    'src/pages/file-with-invalid-ext',
    'src/components/fileWithInvalidExt',
    'src/components/sidebar/sidebar-link.module.css',
  ]

  it.each(notValidFiles)('"%s"', (fileName) => {
    const pluginTransform = getTransform()

    const code = [
      "import { createRoute, dynamic } from 'tuono'",
      "const IndexImport = dynamic(() => import('./../src/routes/index'))",
    ].join('\n')

    const result = pluginTransform(code, fileName)

    expect(result).toBeUndefined()
  })
})
