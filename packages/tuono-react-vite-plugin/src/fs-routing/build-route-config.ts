import type { RouteNode } from '../types'

import { spaces } from './utils'

export function buildRouteConfig(nodes: Array<RouteNode>, depth = 1): string {
  const children = nodes.map((node) => {
    const route = `${node.variableName as string}Route`

    if (node.children?.length) {
      const childConfigs = buildRouteConfig(node.children, depth + 1)
      return `${route}.addChildren([${spaces(depth * 4)}${childConfigs}])`
    }

    return route
  })

  return children.filter(Boolean).join(`,`)
}
