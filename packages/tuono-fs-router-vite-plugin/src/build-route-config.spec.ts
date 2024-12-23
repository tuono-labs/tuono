import { describe, it, expect } from 'vitest'

import { buildRouteConfig } from './build-route-config'

const routes = [
  {
    filePath: 'posts/my-post.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/my-post.tsx',
    routePath: '/posts/my-post',
    variableName: 'PostsMyPost',
    parent: {
      filePath: 'posts/__root.tsx',
      fullPath:
        '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__root.tsx',
      routePath: '/posts/__root',
      variableName: 'Postsroot',
      path: '/posts/__root',
      cleanedPath: '/posts',
      children: undefined,
    },
    path: '/posts/my-post',
    cleanedPath: '/posts/my-post',
  },
  {
    filePath: 'posts/index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/index.tsx',
    routePath: '/posts/',
    variableName: 'PostsIndex',
    parent: {
      filePath: 'posts/__root.tsx',
      fullPath:
        '/home/valerio/Documents/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__root.tsx',
      routePath: '/posts/__root',
      variableName: 'Postsroot',
      path: '/posts/__root',
      cleanedPath: '/posts',
      children: undefined,
    },
    path: '/posts/',
    cleanedPath: '/posts/',
  },
  {
    filePath: 'posts/[post].tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/index.tsx',
    routePath: '/posts/',
    variableName: 'PostspostIndex',
    parent: {
      filePath: 'posts/__root.tsx',
      fullPath:
        '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__root.tsx',
      routePath: '/posts/__root',
      variableName: 'Postsroot',
      path: '/posts/__root',
      cleanedPath: '/posts',
      children: undefined,
    },
    path: '/posts/',
    cleanedPath: '/posts/',
  },
]

describe('buildRouteConfig works', () => {
  it('Should build the correct config', () => {
    const config = buildRouteConfig(routes)
    expect(config).toStrictEqual(
      'PostsMyPostRoute,PostsIndexRoute,PostspostIndexRoute',
    )
  })
})
