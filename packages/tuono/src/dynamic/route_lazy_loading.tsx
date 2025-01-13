import * as React from 'react'
import type { ReactElement } from 'react'

import type { RouteComponent } from 'tuono-router'

type ImportFn = () => Promise<{ default: RouteComponent }>

export const __tuono__internal__lazyLoadComponent = (
  factory: ImportFn,
): RouteComponent => {
  let LoadedComponent: RouteComponent | undefined
  const LazyComponent = React.lazy(factory) as unknown as RouteComponent

  const loadComponent = (): Promise<void> =>
    factory().then((module) => {
      LoadedComponent = module.default
    })

  const Component = (
    props: React.ComponentProps<RouteComponent>,
  ): ReactElement =>
    React.createElement(LoadedComponent || LazyComponent, props)

  Component.preload = loadComponent

  return Component
}
