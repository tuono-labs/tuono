import { describe, it, expect } from 'vitest'

import { sortRouteNodes } from './sort-route-nodes'

const routes = [
  {
    filePath: 'index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/index.tsx',
    routePath: '/',
    variableName: 'Index',
    path: '/',
    cleanedPath: '/',
  },
  {
    filePath: 'about.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/about.tsx',
    routePath: '/about',
    variableName: 'About',
    path: '/about',
    cleanedPath: '/about',
  },
  {
    filePath: '__root.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/__root.tsx',
    routePath: '/__root',
    variableName: 'root',
  },
  {
    filePath: 'posts/[post].tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/[post].tsx',
    routePath: '/posts/[post]',
    variableName: 'Postspost',
  },
  {
    filePath: 'posts/my-post.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/my-post.tsx',
    routePath: '/posts/my-post',
    variableName: 'PostsMyPost',
  },
  {
    filePath: 'posts/index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/index.tsx',
    routePath: '/posts/',
    variableName: 'PostsIndex',
  },
  {
    filePath: 'posts/__root.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__root.tsx',
    routePath: '/posts/__root',
    variableName: 'Postsroot',
  },
]

const expectedSorting = [
  {
    filePath: 'index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/index.tsx',
    routePath: '/',
    variableName: 'Index',
    path: '/',
    cleanedPath: '/',
  },
  {
    filePath: 'about.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/about.tsx',
    routePath: '/about',
    variableName: 'About',
    path: '/about',
    cleanedPath: '/about',
  },
  {
    filePath: 'posts/__root.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/__root.tsx',
    routePath: '/posts/__root',
    variableName: 'Postsroot',
  },
  {
    filePath: 'posts/my-post.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/my-post.tsx',
    routePath: '/posts/my-post',
    variableName: 'PostsMyPost',
  },
  {
    filePath: 'posts/index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/index.tsx',
    routePath: '/posts/',
    variableName: 'PostsIndex',
  },
  {
    filePath: 'posts/[post].tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root-dynamic/routes/posts/[post].tsx',
    routePath: '/posts/[post]',
    variableName: 'Postspost',
  },
]

describe('sortRouteNodes works', () => {
  it('Should correctly sort the nodes', () => {
    const sorted = sortRouteNodes(routes)
    expect(sorted).toStrictEqual(expectedSorting)
  })
})
