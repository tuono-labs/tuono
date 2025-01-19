import type { ReactNode, ComponentType } from 'react'

export interface Segment {
  type: 'pathname' | 'param' | 'wildcard'
  value: string
}

export interface ServerRouterInfo {
  href: string
  pathname: string
  searchStr: string
}

export interface ServerProps<TProps = unknown> {
  router: ServerRouterInfo
  props: TProps
  jsBundles: Array<string>
  cssBundles: Array<string>
  mode: 'Dev' | 'Prod'
}

export interface RouteProps<TData = unknown> {
  data: TData
  isLoading: boolean

  children?: ReactNode
}

export type RouteComponent = ComponentType<RouteProps> & {
  preload: () => void
}
