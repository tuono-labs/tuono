import type { RouteNode } from '../types'

import { multiSortBy } from './utils'
import { LAYOUT_PATH_ID } from './constants'

// Routes need to be sorted in order to iterate over the handleNode fn
// with first the items that might be parent routes
export const sortRouteNodes = (routes: Array<RouteNode>): Array<RouteNode> =>
  multiSortBy(routes, [
    (d): number => (d.routePath === '/' ? -1 : 1),
    (d): number => d.routePath.split('/').length,
    // Dynamic route
    (d): number => (d.routePath.endsWith(']') ? 1 : -1),
    (d): number => (d.filePath.match(/[./]index[.]/) ? 1 : -1),
    (d): number => (d.filePath.match(/[./]route[.]/) ? -1 : 1),
    (d): number => (d.routePath.endsWith('/') ? -1 : 1),
    (d): string => d.routePath,
  ]).filter((d) => ![`/${LAYOUT_PATH_ID}`].includes(d.routePath || ''))
