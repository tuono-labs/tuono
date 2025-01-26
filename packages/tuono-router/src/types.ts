import type { ReactNode, ComponentType } from 'react'

export interface Segment {
  type: 'pathname' | 'param' | 'wildcard'
  value: string
}

/**
 * Provided by the rust server and used in the ssr env
 * @see tuono {@link ServerPayloadLocation}
 */
export interface ServerInitialLocation {
  href: string
  pathname: string
  searchStr: string
}

export interface RouteProps<TData = unknown> {
  data: TData

  isLoading: boolean

  children?: ReactNode
}

export type RouteComponent = ComponentType<RouteProps> & {
  preload: () => void
}
