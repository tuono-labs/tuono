import * as React from 'react'
import type { ReactElement } from 'react'

import type { RouteComponent } from './types'

type ImportFn = () => Promise<{ default: RouteComponent }>

/**
 * Helper function to lazy load any component.
 *
 * The function acts exactly like React.lazy function but also renders the component on the server.
 * If you want to just load the component client side use directly the react's lazy function.
 *
 * It can be wrapped within a React.Suspense component in order to handle its loading state.
 */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const dynamic = (importFn: ImportFn): React.JSX.Element => {
  /**
   *
   * This function is just a placeholder. The real work is done by the bundler.
   * The custom babel plugin will create two different bundles for the client and the server.
   *
   * The client will import the React's lazy function while the server will statically
   * import the file.
   *
   * Example:
   *
   * // User code
   * import { dynamic } from 'tuono'
   * const MyComponent = dynamic(() => import('./my-component'))
   *
   * // Client side generated code
   * import { lazy } from 'react'
   * const MyComponent = lazy(() => import('./my-component'))
   *
   * // Server side generated code
   * import MyComponent from './my-component'
   *
   * Check the `lazy-fn-vite-plugin` package for more
   */
  return <></>
}

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
