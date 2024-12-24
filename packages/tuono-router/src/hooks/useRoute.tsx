import type { Route } from '../route'

import { useRouterContext } from '../components/RouterContext'

const DYNAMIC_PATH_REGEX = /\[(.*?)\]/

/**
 * In order to correctly handle pathnames that might finish with a slash
 * we first sanitize them by removing the final slash.
 */
export function sanitizePathname(pathname: string): string {
  if (pathname.endsWith('/') && pathname !== '/') {
    return pathname.substring(0, pathname.length - 1)
  }

  return pathname
}

/*
 * This hook is also implemented on server side to match the bundle
 * file to load at the first rendering.
 *
 * File: crates/tuono_lib/src/payload.rs
 *
 * Optimizations should occour on both
 */
export default function useRoute(pathname?: string): Route | undefined {
  const {
    router: { routesById },
  } = useRouterContext()

  if (!pathname) return

  pathname = sanitizePathname(pathname)

  if (routesById[pathname]) return routesById[pathname]

  const dynamicRoutes = Object.keys(routesById).filter((route) =>
    DYNAMIC_PATH_REGEX.test(route),
  )

  if (!dynamicRoutes.length) return

  const pathSegments = pathname.split('/').filter(Boolean)

  let match = undefined

  // TODO: Check algo efficiency
  for (const dynamicRoute of dynamicRoutes) {
    const dynamicRouteSegments = dynamicRoute.split('/').filter(Boolean)

    const routeSegmentsCollector: Array<string> = []

    for (let i = 0; i < dynamicRouteSegments.length; i++) {
      if (dynamicRouteSegments[i]?.startsWith('[...')) {
        routeSegmentsCollector.push(dynamicRouteSegments[i] ?? '')
        match = `/${routeSegmentsCollector.join('/')}`
        break
      }
      if (
        dynamicRouteSegments[i] === pathSegments[i] ||
        DYNAMIC_PATH_REGEX.test(dynamicRouteSegments[i] || '')
      ) {
        routeSegmentsCollector.push(dynamicRouteSegments[i] ?? '')
      } else {
        break
      }
    }

    if (routeSegmentsCollector.length === pathSegments.length) {
      match = `/${routeSegmentsCollector.join('/')}`
      break
    }
  }

  if (!match) return
  return routesById[match]
}
