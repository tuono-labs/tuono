import path from 'path'

import { describe, expect, it } from 'vitest'

import { getStylesForComponentId } from './styles'

describe('getStylesForComponentId', () => {
  it('falls back to mdx route files when fetching critical css', async () => {
    const calls: string[] = []
    const expectedRoutePath = path.join(process.cwd(), 'src/routes/about')
    const aboutMdxNode = {
      file: `${expectedRoutePath}.mdx`,
      url: '/src/routes/about.mdx',
      importedModules: new Set(),
      ssrTransformResult: { deps: [] },
    }

    const viteDevServer = {
      moduleGraph: {
        getModuleByUrl: async (url: string) => {
          calls.push(url)
          return url === expectedRoutePath ? undefined : aboutMdxNode
        },
      },
      transformRequest: async (url: string) => {
        calls.push(`transform:${url}`)
        return undefined
      },
      ssrLoadModule: async () => ({ default: '' }),
    }

    const css = await getStylesForComponentId(
      viteDevServer as never,
      'about',
      {},
    )

    expect(calls).toContain(expectedRoutePath)
    expect(calls).toContain(`${expectedRoutePath}.mdx`)
    expect(css).toBeUndefined()
  })
})
