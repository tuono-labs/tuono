import fs from 'fs/promises'

import { describe, it, expect } from 'vitest'

import { routeGenerator } from '../src/fs-routing/generator'

describe('generator works', async () => {
  const folderNames = await fs.readdir(`${process.cwd()}/tests/generator`)

  it.each(folderNames)(
    'should wire-up the routes for a "%s" tree',
    async (folderName) => {
      const testDirPath = `${process.cwd()}/tests/generator/${folderName}`

      await routeGenerator({
        folderName: `${testDirPath}/routes`,
        generatedRouteTree: `${testDirPath}/routeTree.gen.ts`,
      })

      const generatedFilePath = `${testDirPath}/routeTree.gen.ts`
      const expectedFilePath = `${testDirPath}/routeTree.expected.ts`

      const generatedFileContent = await fs.readFile(generatedFilePath, 'utf-8')

      await expect(generatedFileContent).toMatchFileSnapshot(
        expectedFilePath,
        `${generatedFilePath} content should be equal to ${expectedFilePath}`,
      )
    },
  )
})
