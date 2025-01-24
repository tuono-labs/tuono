import type { JSX, ReactNode } from 'react'
import { createContext, useContext, useMemo } from 'react'

import type { ServerPayload } from '../types'

const isServerSide = typeof window === 'undefined'

interface TuonoContextValue {
  serverPayload: ServerPayload
}

const TuonoContext = createContext({} as TuonoContextValue)

interface TuonoContextProviderProps {
  serverPayload?: ServerPayload

  children: ReactNode
}

export function TuonoContextProvider(
  props: TuonoContextProviderProps,
): JSX.Element {
  const { serverPayload, children } = props

  const contextValue = useMemo(() => {
    const _serverPayload = isServerSide
      ? serverPayload
      : window.__TUONO_SERVER_PAYLOAD__

    return {
      /** Maybe this logic should be integrated using defaults */
      serverPayload: _serverPayload,
    } as TuonoContextValue
  }, [serverPayload])

  return <TuonoContext value={contextValue}>{children}</TuonoContext>
}

export function useTuonoContextServerPayload(): TuonoContextValue['serverPayload'] {
  return useContext(TuonoContext).serverPayload
}
