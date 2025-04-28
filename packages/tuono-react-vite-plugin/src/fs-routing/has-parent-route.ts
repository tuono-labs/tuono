import type { RouteNode } from '../types'

import { LAYOUT_PATH_ID } from './constants'
import { multiSortBy } from './utils'

export function hasParentRoute(
  routes: Array<RouteNode>,
  node: RouteNode,
  routePathToCheck = '/',
): RouteNode | null {
  const segments = routePathToCheck.split('/')
  segments.pop() // Remove the last segment
  const parentRoutePath = segments.join('/')

  if (!parentRoutePath || parentRoutePath === '/') {
    return null
  }

  const sortedNodes = multiSortBy(routes, [
    (d): number => d.routePath.length * -1,
    (d): string | undefined => d.variableName,
  ])
    // Exclude base __layout file
    .filter((d) => d.routePath !== `/${LAYOUT_PATH_ID}`)

  for (const route of sortedNodes) {
    if (route.routePath === '/') continue

    if (
      route.routePath.startsWith(parentRoutePath) &&
      route.routePath.endsWith(LAYOUT_PATH_ID)
    ) {
      return route
    }
  }

  return hasParentRoute(routes, node, parentRoutePath)
}
