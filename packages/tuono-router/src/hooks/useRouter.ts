import React from 'react'

import { useRouterContext } from '../components/RouterContext'

interface PushOptions {
  /**
   * If "false" the scroll offset will be kept across page navigation. Default "true"
   */
  scroll?: boolean
}

interface UseRouterResult {
  /**
   * Redirects to the path passed as argument updating the browser history.
   */
  push: (path: string, opt?: PushOptions) => void

  /**
   * This object contains all the query params of the current route
   */
  query: Record<string, string>

  /**
   * Returns the current pathname
   */
  pathname: string
}

export const useRouter = (): UseRouterResult => {
  const { location, updateLocation } = useRouterContext()

  const push = React.useCallback(
    (path: string, opt?: PushOptions): void => {
      const { scroll = true } = opt || {}
      const url = new URL(path, window.location.origin)

      updateLocation({
        href: url.href,
        pathname: url.pathname,
        search: Object.fromEntries(url.searchParams),
        searchStr: url.search,
        hash: url.hash,
      })
      history.pushState(path, '', path)

      if (scroll) {
        window.scroll(0, 0)
      }
    },
    [updateLocation],
  )

  return {
    push,
    query: location.search,
    pathname: location.pathname,
  }
}
