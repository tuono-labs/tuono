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

/**
 * @warning THIS SHOULD NOT BE EXPOSED TO USERLAND
 *
 * @see https://github.com/tuono-labs/tuono/issues/410
 */
export function TuonoContextProvider({
  serverPayload,
  children,
}: TuonoContextProviderProps): JSX.Element {
  const contextValue: TuonoContextValue = useMemo(() => {
    // At least one of these two should be defined
    const _serverPayload = (
      isServerSide ? serverPayload : window.__TUONO_SERVER_PAYLOAD__
    ) as ServerPayload

    return {
      // Maybe this logic should be integrated using defaults
      serverPayload: _serverPayload,
    }
  }, [serverPayload])

  return <TuonoContext value={contextValue}>{children}</TuonoContext>
}

/**
 * @warning THIS SHOULD NOT BE EXPOSED TO USERLAND
 */
export function useTuonoContextServerPayload(): TuonoContextValue['serverPayload'] {
  return useContext(TuonoContext).serverPayload
}
