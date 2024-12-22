import type { ComponentType as ReactComponentType, ReactNode } from 'react'

export interface Segment {
  type: 'pathname' | 'param' | 'wildcard'
  value: string
}

export interface ServerProps<TProps = unknown> {
  router: {
    href: string
    pathname: string
    searchStr: string
  }
  props: TProps
}

export interface RouteProps<TData = unknown> {
  data: TData
  isLoading: boolean

  children?: ReactNode
}

export type RouteComponent = ReactComponentType<RouteProps> & {
  preload: () => void
}
