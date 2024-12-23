import { afterEach, describe, expect, test, vi } from 'vitest'
import { cleanup } from '@testing-library/react'

import useRoute from './useRoute'

describe('Test useRoute fn', () => {
  afterEach(cleanup)

  test('match routes by ids', () => {
    vi.mock('./useInternalRouter.tsx', () => ({
      useInternalRouter: (): {
        router: { routesById: Record<string, { id: string }> }
      } => {
        return {
          router: {
            routesById: {
              '/': { id: '/' },
              '/about': { id: '/about' },
              '/posts': { id: '/posts' }, // posts/index
              '/posts/[post]': { id: '/posts/[post]' },
              '/posts/defined-post': { id: '/posts/defined-post' },
              '/posts/[post]/[comment]': { id: '/posts/[post]/[comment]' },
              '/blog/[...catch_all]': { id: '/blog/[...catch_all]' },
            },
          },
        }
      },
    }))

    expect(useRoute('/')?.id).toBe('/')
    expect(useRoute('/not-found')?.id).toBe(undefined)
    expect(useRoute('/about')?.id).toBe('/about')
    expect(useRoute('/posts/')?.id).toBe('/posts')
    expect(useRoute('/posts/dynamic-post')?.id).toBe('/posts/[post]')
    expect(useRoute('/posts/defined-post')?.id).toBe('/posts/defined-post')
    expect(useRoute('/posts/dynamic-post/dynamic-comment')?.id).toBe(
      '/posts/[post]/[comment]',
    )
    expect(useRoute('/blog/catch_all')?.id).toBe('/blog/[...catch_all]')
    expect(useRoute('/blog')?.id).toBe('/blog/[...catch_all]')
    expect(useRoute('/blog/catch_all/catch_all')?.id).toBe(
      '/blog/[...catch_all]',
    )
  })
})
