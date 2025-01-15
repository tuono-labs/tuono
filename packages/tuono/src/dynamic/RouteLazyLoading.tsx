import { lazy, createElement } from 'react'
import type { ReactElement } from 'react'

import type { RouteComponent } from 'tuono-router'

type ImportFn = () => Promise<{ default: RouteComponent }>

export const RouteLazyLoading = (factory: ImportFn): RouteComponent => {
  let LoadedComponent: RouteComponent | undefined
  const LazyComponent = lazy<RouteComponent>(factory)

  const loadComponent = (): Promise<void> =>
    factory().then((module) => {
      LoadedComponent = module.default
    })

  const Component = (
    props: React.ComponentProps<RouteComponent>,
  ): ReactElement => createElement(LoadedComponent || LazyComponent, props)

  Component.preload = loadComponent

  return Component
}
