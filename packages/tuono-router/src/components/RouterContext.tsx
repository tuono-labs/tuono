import {
  createContext,
  useState,
  useEffect,
  useContext,
  useMemo,
  useCallback,
} from 'react'
import type { ReactNode } from 'react'

import type { Router } from '../router'
import type { ServerInitialLocation } from '../types'

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
  isTransitioning: boolean
  updateLocation: (loc: ParsedLocation) => void
  stopTransitioning: () => void
}

const RouterContext = createContext({} as RouterContextValue)

function getInitialLocation(
  serverPayloadLocation: ServerInitialLocation,
): ParsedLocation {
  if (isServerSide) {
    return {
      pathname: serverPayloadLocation.pathname || '',
      hash: '',
      href: serverPayloadLocation.href || '',
      searchStr: serverPayloadLocation.searchStr || '',
      search: Object.fromEntries(
        new URLSearchParams(serverPayloadLocation.searchStr),
      ),
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
  serverInitialLocation: ServerInitialLocation
  children: ReactNode
}

export function RouterContextProvider({
  router,
  serverInitialLocation,
  children,
}: RouterContextProviderProps): ReactNode {
  // Allow the router to update options on the router instance
  router.update({ ...router.options } as Parameters<typeof router.update>[0])

  const [location, setLocation] = useState<ParsedLocation>(() =>
    getInitialLocation(serverInitialLocation),
  )
  // Global state to track whether a page transition is in progress.
  // Set to `false` once the page is fully loaded, including server-side data.
  const [isTransitioning, setIsTransitioning] = useState<boolean>(false)

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

  const updateLocation = useCallback((newLocation: ParsedLocation): void => {
    setIsTransitioning(true)
    setLocation(newLocation)
  }, [])

  const stopTransitioning = useCallback((): void => {
    setIsTransitioning(false)
  }, [])

  const contextValue: RouterContextValue = useMemo(
    () => ({
      router,
      location,
      isTransitioning,
      updateLocation,
      stopTransitioning,
    }),
    [location, router, isTransitioning, updateLocation, stopTransitioning],
  )

  return (
    <RouterContext.Provider value={contextValue}>
      {children}
    </RouterContext.Provider>
  )
}

/**
 * @warning THIS SHOULD NOT BE EXPOSED TO USERLAND
 */
export function useRouterContext(): RouterContextValue {
  return useContext(RouterContext)
}
