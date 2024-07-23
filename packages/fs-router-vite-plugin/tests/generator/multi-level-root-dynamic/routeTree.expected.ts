// This file is auto-generated by Tuono

import { createRoute, dynamic } from 'tuono'

import RootImport from './routes/__root'

const PostsrootImport = dynamic(() => import('./routes/posts/__root'))
const AboutImport = dynamic(() => import('./routes/about'))
const IndexImport = dynamic(() => import('./routes/index'))
const PostspostImport = dynamic(() => import('./routes/posts/[post]'))
const PostsIndexImport = dynamic(() => import('./routes/posts/index'))
const PostsMyPostImport = dynamic(() => import('./routes/posts/my-post'))

const rootRoute = createRoute({ isRoot: true, component: RootImport })

const Postsroot = createRoute({ component: PostsrootImport, isRoot: true })
const About = createRoute({ component: AboutImport })
const Index = createRoute({ component: IndexImport })
const Postspost = createRoute({ component: PostspostImport })
const PostsIndex = createRoute({ component: PostsIndexImport })
const PostsMyPost = createRoute({ component: PostsMyPostImport })

// Create/Update Routes

const PostsrootRoute = Postsroot.update({
  path: '/posts',
  getParentRoute: () => rootRoute,
})

const AboutRoute = About.update({
  path: '/about',
  getParentRoute: () => rootRoute,
})

const IndexRoute = Index.update({
  path: '/',
  getParentRoute: () => rootRoute,
})

const PostspostRoute = Postspost.update({
  path: '/posts/[post]',
  getParentRoute: () => PostsrootRoute,
})

const PostsIndexRoute = PostsIndex.update({
  path: '/posts/',
  getParentRoute: () => PostsrootRoute,
})

const PostsMyPostRoute = PostsMyPost.update({
  path: '/posts/my-post',
  getParentRoute: () => PostsrootRoute,
})

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  IndexRoute,
  AboutRoute,
  PostsrootRoute.addChildren([
    PostsMyPostRoute,
    PostsIndexRoute,
    PostspostRoute,
  ]),
  PostsMyPostRoute,
  PostsIndexRoute,
  PostspostRoute,
])
