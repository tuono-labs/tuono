import React from 'react'

import { useRouterContext } from '../components/RouterContext'

type NavigationType = 'pushState' | 'replaceState'

interface NavigateOptions {
  /**
   * If "false" the scroll offset will be kept across page navigation. Default "true"
   */
  scroll?: boolean
}

interface UseRouterResult {
  /**
   * Redirects to the path passed as argument updating the browser history.
   */
  push: (path: string, opt?: NavigateOptions) => void

  /**
   * Redirects to the path passed as argument replacing the current history
   * entry.
   */
  replace: (path: string, opt?: NavigateOptions) => void

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

  const navigate = React.useCallback(
    (type: NavigationType, path: string, opt?: NavigateOptions): void => {
      const { scroll = true } = opt || {}
      const url = new URL(path, window.location.origin)

      updateLocation({
        href: url.href,
        pathname: url.pathname,
        search: Object.fromEntries(url.searchParams),
        searchStr: url.search,
        hash: url.hash,
      })

      history[type](path, '', path)

      if (scroll) {
        window.scroll(0, 0)
      }
    },
    [updateLocation],
  )

  const push = React.useCallback(
    (path: string, opt?: NavigateOptions): void => {
      navigate('pushState', path, opt)
    },
    [navigate],
  )

  const replace = React.useCallback(
    (path: string, opt?: NavigateOptions): void => {
      navigate('replaceState', path, opt)
    },
    [navigate],
  )

  return {
    push,
    replace,
    query: location.search,
    pathname: location.pathname,
  }
}
