import React, { useState, useEffect } from 'react'
import type { ReactNode } from 'react'

import type { Router } from '../router'
import type { ServerRouterInfo } from '../types'

export interface ParsedLocation {
  href: string
  pathname: string
  search: Record<string, string>
  searchStr: string
  hash: string
}

export interface RouterContextType {
  router: Router
  location: ParsedLocation
  updateLocation: (loc: ParsedLocation) => void
}

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const routerContext = React.createContext<RouterContextType>(null!)

const TUONO_CONTEXT_GLOBAL_NAME = '__TUONO_CONTEXT__'

function getRouterContext(): React.Context<RouterContextType> {
  if (typeof document === 'undefined') {
    return routerContext
  }

  if (window[TUONO_CONTEXT_GLOBAL_NAME]) {
    return window[TUONO_CONTEXT_GLOBAL_NAME]
  }

  window[TUONO_CONTEXT_GLOBAL_NAME] = routerContext

  return routerContext
}

interface RouterContextProviderProps {
  router: Router
  children: ReactNode
  serverSideProps?: ServerRouterInfo
}

function getInitialLocation(
  serverSideProps?: ServerRouterInfo,
): ParsedLocation {
  if (typeof document === 'undefined') {
    return {
      pathname: serverSideProps?.pathname || '',
      hash: '',
      href: serverSideProps?.href || '',
      searchStr: serverSideProps?.searchStr || '',
      // TODO: Polyfill URLSearchParams
      search: {},
    }
  }

  const { location } = window
  return {
    pathname: location.pathname,
    hash: location.hash,
    href: location.href,
    searchStr: location.search,
    search: Object.fromEntries(new URLSearchParams(location.search)),
  }
}

export function RouterContextProvider({
  router,
  children,
  serverSideProps,
}: RouterContextProviderProps): ReactNode {
  // Allow the router to update options on the router instance
  router.update({ ...router.options } as Parameters<typeof router.update>[0])

  const [location, updateLocation] = useState<ParsedLocation>(
    getInitialLocation(serverSideProps),
  )

  /**
   * Listen browser navigation events
   */
  useEffect(() => {
    const updateLocationOnPopStateChange = ({
      target,
    }: PopStateEvent): void => {
      const { location: targetLocation } = target as typeof window
      const { pathname, hash, href, search } = targetLocation

      updateLocation({
        pathname,
        hash,
        href,
        searchStr: search,
        search: Object.fromEntries(new URLSearchParams(search)),
      })
    }

    window.addEventListener('popstate', updateLocationOnPopStateChange)

    return (): void => {
      window.removeEventListener('popstate', updateLocationOnPopStateChange)
    }
  }, [updateLocation])

  const RouterContext = getRouterContext()

  const context: RouterContextType = {
    router,
    location,
    updateLocation,
  }

  return (
    <RouterContext.Provider value={context}>{children}</RouterContext.Provider>
  )
}

export function useInternalRouter(): RouterContextType {
  return React.useContext(getRouterContext())
}
