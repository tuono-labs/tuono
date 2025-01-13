import type { ReactNode } from 'react'

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
}

export interface RouteProps<TData = unknown> {
  data: TData
  isLoading: boolean

  children?: ReactNode
}
