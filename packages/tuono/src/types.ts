import type { ReactNode } from 'react'

import type { TuonoConfigServer } from './config'

/**
 * Provided by the rust server and used in the ssr env
 * @see tuono-router {@link ServerInitialLocation}
 */
export interface ServerPayloadLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @see crates/tuono_lib/src/payload.rs
 */
export type ServerPayload<TData = unknown> = {
  location: ServerPayloadLocation

  data: TData
} & (
  | {
      mode: 'Prod'
      jsBundles: Array<string>
      cssBundles: Array<string>
    }
  | {
      mode: 'Dev'
      devServerConfig?: TuonoConfigServer
    }
)

export type TuonoRouteProps<TData> =
  | {
      data: null
      isLoading: true
    }
  | {
      data: TData
      isLoading: false
    }

export interface TuonoLayoutProps {
  children: ReactNode
}
