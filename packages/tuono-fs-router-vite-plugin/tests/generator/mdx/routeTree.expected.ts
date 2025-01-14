// This file is auto-generated by Tuono

import { createRoute, __tuono__internal__lazyLoadRoute } from 'tuono'

import RootLayoutImport from './routes/__layout'

const AboutImport = __tuono__internal__lazyLoadRoute(
  () => import('./routes/about.mdx'),
)
const IndexImport = __tuono__internal__lazyLoadRoute(
  () => import('./routes/index'),
)

const rootRoute = createRoute({ isRoot: true, component: RootLayoutImport })

const About = createRoute({ component: AboutImport })
const Index = createRoute({ component: IndexImport })

// Create/Update Routes

const AboutRoute = About.update({
  path: '/about',
  getParentRoute: () => rootRoute,
})

const IndexRoute = Index.update({
  path: '/',
  getParentRoute: () => rootRoute,
})

// Create and export the route tree

export const routeTree = rootRoute.addChildren([IndexRoute, AboutRoute])
