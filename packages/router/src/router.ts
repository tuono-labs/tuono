import type { ComponentType as ReactComponentType } from 'react'

import { trimPath, trimPathRight } from './utils'
import type { Route } from './route'

type RouteTree = Route

interface CreateRouterOptions {
  routeTree: RouteTree
  basePath?: string
  options?: RouterOptions
}

interface RouterOptions {
  component?: ReactComponentType
  hasHandler?: boolean
  routeTree?: RouteTree
}

export function createRouter(options: CreateRouterOptions): Router {
  return new Router(options)
}

export class Router {
  options?: RouterOptions
  basePath = '/'
  routeTree?: RouteTree

  isServer = typeof document === 'undefined'

  routesById: Record<string, Route> = {}

  routesByPath: Record<string, Route> = {}

  constructor(options: CreateRouterOptions) {
    this.update({
      ...options,
    })

    if (!this.isServer) {
      window.__TUONO__ROUTER__ = this
    }
  }

  update = (newOptions: CreateRouterOptions): void => {
    this.options = {
      ...this.options,
      ...newOptions,
    }

    this.#updateBasePath(newOptions.basePath)

    if (this.options.routeTree !== this.routeTree) {
      this.routeTree = this.options.routeTree
      this.#buildRouteTree()
    }
  }

  #buildRouteTree = (): void => {
    const recurseRoutes = (childRoutes: Array<Route>): void => {
      childRoutes.forEach((route: Route, i: number) => {
        route.init(i)

        this.routesById[route.id || ''] = route

        if (!route.isRoot && route.options.path) {
          const trimmedFullPath = trimPathRight(route.fullPath)
          if (
            !this.routesByPath[trimmedFullPath] ||
            route.fullPath.endsWith('/')
          ) {
            this.routesByPath[trimmedFullPath] = route
          }
        }

        const children = route.children
        if (children?.length) {
          recurseRoutes(children)
        }
      })
    }

    recurseRoutes([this.routeTree as Route])
  }

  #updateBasePath = (basePath?: string): void => {
    if (!this.basePath || (basePath && basePath !== basePath)) {
      if (basePath === undefined || basePath === '' || basePath === '/') {
        this.basePath = '/'
      } else {
        this.basePath = `/${trimPath(basePath)}`
      }
    }
  }
}
