import { useState, useEffect, useRef } from 'react'

import type { Route } from '../route'
import { fromUrlToParsedLocation } from '../utils/from-url-to-parsed-location'

import { useRouterContext } from '../components/RouterContext'

interface TuonoApi {
  data?: unknown
  info: {
    redirect_destination?: string
  }
}

const fetchClientSideData = async (): Promise<TuonoApi> => {
  const res = await fetch(`/__tuono/data${location.pathname}`)
  const data = (await res.json()) as TuonoApi
  return data
}

interface UseServerPayloadDataResult<TData> {
  data: TData
}

/*
 * Use the props provided by the SSR and dehydrate the
 * props for client side usage.
 *
 * In case is a client fetch the remote data API
 */
export function useServerPayloadData<TServerPayloadData>(
  route: Route,
  // User defined data
  serverInitialData: TServerPayloadData,
): UseServerPayloadDataResult<TServerPayloadData> {
  const isFirstRendering = useRef<boolean>(true)
  const { location, updateLocation, stopTransitioning } = useRouterContext()

  const [data, setData] = useState<TServerPayloadData | undefined>(
    serverInitialData,
  )

  useEffect(() => {
    // First loading just dehydrate since the
    // props are already bundled by the SSR
    if (isFirstRendering.current) {
      isFirstRendering.current = false
      stopTransitioning()
      return
    }
    // After client side routing load again the remote data
    if (route.options.hasHandler) {
      // The error management is already handled inside the IIFE
      // eslint-disable-next-line @typescript-eslint/no-floating-promises
      ;(async (): Promise<void> => {
        try {
          const response = await fetchClientSideData()
          if (response.info.redirect_destination) {
            const parsedLocation = fromUrlToParsedLocation(
              response.info.redirect_destination,
            )

            history.pushState(
              parsedLocation.pathname,
              '',
              parsedLocation.pathname,
            )

            updateLocation(parsedLocation)
            return
          }
          setData(response.data as TServerPayloadData)
        } catch (error) {
          throw Error('Failed loading Server Side Data', { cause: error })
        } finally {
          stopTransitioning()
        }
      })()
    } else {
      stopTransitioning()
    }

    // Clean up the data when changing route
    return (): void => {
      setData(undefined)
    }
  }, [
    location.pathname,
    route.options.hasHandler,
    updateLocation,
    stopTransitioning,
  ])

  return { data: data as TServerPayloadData }
}
