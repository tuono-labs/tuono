import { describe, it, expect } from 'vitest'

import { hasParentRoute } from './has-parent-route'

const routes = [
  {
    filePath: 'posts/[post].tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/[post].tsx',
    routePath: '/posts/[post]',
    variableName: 'Postspost',
    path: '/posts/[post]',
    cleanedPath: '/posts/[post]',
  },
  {
    filePath: 'posts/__layout.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/__layout.tsx',
    routePath: '/posts/__layout',
    variableName: 'Postsroot',
    path: '/posts/__layout',
    cleanedPath: '/posts',
  },
  {
    filePath: 'index.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/index.tsx',
    routePath: '/',
    variableName: 'Index',
    path: '/',
    cleanedPath: '/',
  },
  {
    filePath: 'posts/my-post.tsx',
    fullPath:
      '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/my-post.tsx',
    routePath: '/posts/my-post',
    variableName: 'PostsMyPost',
    path: '/posts/my-post',
    cleanedPath: '/posts/my-post',
  },
]

const parent = {
  filePath: 'posts/__layout.tsx',
  fullPath:
    '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/__layout.tsx',
  routePath: '/posts/__layout',
  variableName: 'Postsroot',
  path: '/posts/__layout',
  cleanedPath: '/posts',
}

const myPost = {
  filePath: 'posts/my-post.tsx',
  fullPath:
    '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/my-post.tsx',
  routePath: '/posts/my-post',
  variableName: 'PostsMyPost',
  path: '/posts/my-post',
  cleanedPath: '/posts/my-post',
}

const dynamicRoute = {
  filePath: 'posts/[post].tsx',
  fullPath:
    '/tuono/packages/tuono-fs-router-vite-plugin/tests/generator/multi-level-root/routes/posts/[post].tsx',
  routePath: '/posts/[post]',
  variableName: 'Postspost',
  path: '/posts/[post]',
  cleanedPath: '/posts/[post]',
}

describe('hasParentRoute works', () => {
  it('should detect parent route', () => {
    const parentRoute = hasParentRoute(routes, myPost, myPost.path)

    expect(parentRoute).toStrictEqual(parent)
  })

  it('should detect parent route for dynamic routes', () => {
    const parentRoute = hasParentRoute(routes, dynamicRoute, dynamicRoute.path)

    expect(parentRoute).toStrictEqual(parent)
  })
})
