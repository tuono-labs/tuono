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
      filePath: 'posts/__layout.tsx',
      fullPath:
        '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__layout.tsx',
      routePath: '/posts/__layout',
      variableName: 'Postsroot',
      path: '/posts/__layout',
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
      filePath: 'posts/__layout.tsx',
      fullPath:
        '/home/valerio/Documents/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__layout.tsx',
      routePath: '/posts/__layout',
      variableName: 'Postsroot',
      path: '/posts/__layout',
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
      filePath: 'posts/__layout.tsx',
      fullPath:
        '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__layout.tsx',
      routePath: '/posts/__layout',
      variableName: 'Postsroot',
      path: '/posts/__layout',
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
