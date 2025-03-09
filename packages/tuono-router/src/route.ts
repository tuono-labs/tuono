import type { RouteComponent } from './types'
import { trimPathLeft, joinPaths } from './utils'

interface RouteOptions {
  id?: string
  isRoot?: boolean
  getParentRoute?: () => Route
  path?: string
  component: RouteComponent
  hasHandler?: boolean
}

export function createRoute(options: RouteOptions): Route {
  return new Route(options)
}

export const ROOT_ROUTE_ID = '__root__'

export class Route {
  options: RouteOptions

  id?: string
  isRoot: boolean
  path?: string
  fullPath!: string

  children?: Array<Route>
  parentRoute?: Route
  originalIndex?: number
  component: RouteComponent

  '$$typeof': symbol

  constructor(options: RouteOptions) {
    this.isRoot = options.isRoot ?? typeof options.getParentRoute !== 'function'
    this.options = options
    this.$$typeof = Symbol.for('react.memo')

    this.component = options.component
  }

  init = (originalIndex: number): void => {
    this.originalIndex = originalIndex

    const isRoot = !this.options.path && !this.options.id

    this.parentRoute = this.options.getParentRoute?.()

    if (isRoot) {
      this.path = ROOT_ROUTE_ID
    }

    let path: undefined | string = isRoot ? ROOT_ROUTE_ID : this.options.path

    // If the path is anything other than an index path, trim it up
    if (path && path !== '/') {
      path = trimPathLeft(path)
    }

    const customId = this.options.id || path

    // Strip the parentId prefix from the first level of children
    let id = isRoot ? ROOT_ROUTE_ID : joinPaths([customId])

    if (path === ROOT_ROUTE_ID) {
      path = '/'
    }

    if (id !== ROOT_ROUTE_ID) {
      id = joinPaths(['/', id])
    }

    const fullPath = id === ROOT_ROUTE_ID ? '/' : path

    this.path = path
    this.id = id
    this.fullPath = fullPath || ''
  }

  addChildren(routes: Array<Route>): this {
    this.children = routes
    return this
  }

  update = (options: RouteOptions): this => {
    Object.assign(this.options, options)
    this.isRoot = options.isRoot || !options.getParentRoute
    return this
  }
}

// TODO: not use yet. To be updated in tuono-fs-router-vite-plugin package
export function createRootRoute(options: RouteOptions): Route {
  return new Route({ ...options, isRoot: true })
}
