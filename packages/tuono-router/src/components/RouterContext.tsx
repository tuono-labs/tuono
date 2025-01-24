import { createContext, useState, useEffect, useContext, useMemo } from 'react'
import type { ReactNode } from 'react'

import type { Router } from '../router'
import type { ServerPayload } from '../types'

const isServerSide = typeof window === 'undefined'

export interface ParsedLocation {
  href: string
  pathname: string
  search: Record<string, string>
  searchStr: string
  hash: string
}

interface RouterContextValue {
  router: Router
  location: ParsedLocation
  serverPayload?: ServerPayload
  updateLocation: (loc: ParsedLocation) => void
}

const RouterContext = createContext({} as RouterContextValue)

function getInitialLocation(
  serverPayloadLocation?: ServerPayload['location'],
): ParsedLocation {
  if (isServerSide) {
    return {
      pathname: serverPayloadLocation?.pathname || '',
      hash: '',
      href: serverPayloadLocation?.href || '',
      searchStr: serverPayloadLocation?.searchStr || '',
      // TODO: Polyfill URLSearchParams
      search: {},
    }
  }

  const { pathname, hash, href, search } = window.location
  return {
    pathname,
    hash,
    href,
    searchStr: search,
    search: Object.fromEntries(new URLSearchParams(search)),
  }
}

interface RouterContextProviderProps {
  router: Router
  children: ReactNode
  serverPayload?: ServerPayload
}

export function RouterContextProvider({
  router,
  children,
  serverPayload,
}: RouterContextProviderProps): ReactNode {
  // Allow the router to update options on the router instance
  router.update({ ...router.options } as Parameters<typeof router.update>[0])

  const [location, setLocation] = useState<ParsedLocation>(() =>
    getInitialLocation(serverPayload?.location),
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

      setLocation({
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
  }, [])

  const contextValue: RouterContextValue = useMemo(
    () => ({
      serverPayload: isServerSide
        ? serverPayload
        : window.__TUONO_SERVER_PAYLOAD__,
      router,
      location,
      updateLocation: setLocation,
    }),
    [location, router, serverPayload],
  )

  return (
    <RouterContext.Provider value={contextValue}>
      {children}
    </RouterContext.Provider>
  )
}

/** @warning This hook should not be exported in user land */
export function useRouterContext(): RouterContextValue {
  return useContext(RouterContext)
}
