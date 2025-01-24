import type { ReactNode, ComponentType } from 'react'

export interface Segment {
  type: 'pathname' | 'param' | 'wildcard'
  value: string
}

/**
 * Provided from the the rust server and use in ssr env
 * @see tuono {@link ServerPayloadLocation}
 */
export interface ServerInitialLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @todo This could be refined using a union with isLoading: true | false.
 *       Using this user should be able to have a better typechecking on their end:
 *       In order to have the correct type they will be forced to add a if (isLoading) to
 *       handle loading status.
 * e.g.:
 * ```ts
 * { isLoading: false; data: TData } | { isLoading: true; data: TData }
 * ```
 *
 * ```ts
 * if (isLoading) {
 *   // data is undefined
 * }
 *
 * // data is TData
 * ```
 *
 */
export interface RouteProps<TData = unknown> {
  data: TData

  isLoading: boolean

  children?: ReactNode
}

export type RouteComponent = ComponentType<RouteProps> & {
  preload: () => void
}
