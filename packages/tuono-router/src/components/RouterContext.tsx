import { createContext, useState, useEffect, useContext, useMemo } from 'react'
import type { ReactNode } from 'react'

import type { Router } from '../router'
import type { ServerRouterInfo, ServerProps } from '../types'

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
  serverSideProps?: ServerProps
  updateLocation: (loc: ParsedLocation) => void
}

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const RouterContext = createContext<RouterContextValue>(null!)

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

interface RouterContextProviderProps {
  router: Router
  children: ReactNode
  serverSideProps?: ServerProps
}

export function RouterContextProvider({
  router,
  children,
  serverSideProps,
}: RouterContextProviderProps): ReactNode {
  // Allow the router to update options on the router instance
  router.update({ ...router.options } as Parameters<typeof router.update>[0])

  const [location, setLocation] = useState<ParsedLocation>(() =>
    getInitialLocation(serverSideProps?.router),
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
      serverSideProps: isServerSide
        ? serverSideProps
        : window.__TUONO_SSR_PROPS__,
      router,
      location,
      updateLocation: setLocation,
    }),
    [location, router, serverSideProps],
  )

  return (
    <RouterContext.Provider value={contextValue}>
      {children}
    </RouterContext.Provider>
  )
}

/** @warning DO NOT EXPORT THIS TO USER LAND */
export function useRouterContext(): RouterContextValue {
  return useContext(RouterContext)
}
