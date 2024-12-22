import { useEffect } from 'react'

import { useRouterStore } from './useRouterStore'

/**
 * This hook is meant to handle just browser related location updates
 * like the back and forward buttons.
 */
export const useListenBrowserUrlUpdates = (): void => {
  const updateLocation = useRouterStore((st) => st.updateLocation)

  useEffect(() => {
    const updateLocationOnPopStateChange = ({
      target,
    }: PopStateEvent): void => {
      const { location } = target as typeof window
      const { pathname, hash, href, search } = location

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
}
