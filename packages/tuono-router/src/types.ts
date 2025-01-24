import type { ReactNode, ComponentType } from 'react'

export interface Segment {
  type: 'pathname' | 'param' | 'wildcard'
  value: string
}

interface ServerPayloadLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @see crates/tuono_lib/src/payload.rs
 * @warning keep in sync with the same interface inside tuono until router is not specialized
 */
export interface ServerPayload<TData = unknown> {
  mode: 'Prod' | 'Dev'

  location: ServerPayloadLocation

  data: TData

  /** Available only on 'Prod' mode */
  jsBundles: Array<string> | null
  cssBundles: Array<string> | null

  /** Available only on 'Dev' mode */
  devServerConfig?: {
    port: number
    host: string
  }
}

export interface RouteProps<TData = unknown> {
  data: TData
  isLoading: boolean

  children?: ReactNode
}

export type RouteComponent = ComponentType<RouteProps> & {
  preload: () => void
}
