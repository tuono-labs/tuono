import { useState, useEffect, useRef } from 'react'

import type { Route } from '../route'
import { fromUrlToParsedLocation } from '../utils/from-url-to-parsed-location'

import { useInternalRouter } from '../components/RouterContext'

const isServer = typeof document === 'undefined'

interface UseServerSidePropsReturn<TData> {
  data: TData
  isLoading: boolean
}

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

/*
 * Use the props provided by the SSR and dehydrate the
 * props for client side usage.
 *
 * In case is a client fetch the remote data API
 */
export function useServerSideProps<T>(
  route: Route,
  // User defined props
  serverSideProps: T,
): UseServerSidePropsReturn<T> {
  const isFirstRendering = useRef<boolean>(true)
  const { location, updateLocation } = useInternalRouter()
  const [isLoading, setIsLoading] = useState<boolean>(
    // Force loading if has handler
    !!route.options.hasHandler &&
      // Avoid loading on the server
      !isServer &&
      // Avoid loading if first rendering
      !isFirstRendering.current,
  )

  const [data, setData] = useState<T | undefined>(
    (serverSideProps ?? window.__TUONO_SSR_PROPS__?.props) as T,
  )

  useEffect(() => {
    // First loading just dehydrate since the
    // props are already bundled by the SSR
    if (isFirstRendering.current) {
      isFirstRendering.current = false
      return
    }
    // After client side routing load again the remote data
    if (route.options.hasHandler) {
      // The error management is already handled inside the IIFE
      // eslint-disable-next-line @typescript-eslint/no-floating-promises
      ;(async (): Promise<void> => {
        setIsLoading(true)
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
          setData(response.data as T)
        } catch (error) {
          throw Error('Failed loading Server Side Data', { cause: error })
        } finally {
          setIsLoading(false)
        }
      })()
    }

    // Clean up the data when changing route
    return (): void => {
      setData(undefined)
    }
  }, [location.pathname, route.options.hasHandler, updateLocation])

  return { isLoading, data: data as T }
}
