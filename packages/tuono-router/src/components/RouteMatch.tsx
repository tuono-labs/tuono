import type { JSX } from 'react'
import { memo, Suspense, useMemo } from 'react'

import type { Mode } from '../types'
import type { Route } from '../route'

import { useServerPayloadData } from '../hooks/useServerPayloadData'

import { useRouterContext } from './RouterContext'
import { CriticalCss } from './CriticalCss'

interface RouteMatchProps<TServerPayloadData = unknown> {
  route: Route
  // User defined server side props
  serverInitialData: TServerPayloadData
  mode?: Mode
}

/**
 * Returns the route match with the root element if exists
 *
 * It handles the fetch of the client side resources
 */
export const RouteMatch = ({
  route,
  serverInitialData,
  mode,
}: RouteMatchProps): JSX.Element => {
  const { data } = useServerPayloadData(route, serverInitialData)
  const { isTransitioning } = useRouterContext()

  // eslint-disable-next-line react-hooks/exhaustive-deps
  const routes = useMemo(() => loadParentComponents(route), [route.id])

  const routeData = isTransitioning ? null : data

  return (
    <TraverseRootComponents
      routes={routes}
      data={routeData}
      isLoading={isTransitioning}
      mode={mode}
    >
      <Suspense>
        <CriticalCss routeFilePath={route.filePath} mode={mode} />
        <route.component data={routeData} isLoading={isTransitioning} />
      </Suspense>
    </TraverseRootComponents>
  )
}

interface TraverseRootComponentsProps<TData = unknown> {
  routes: Array<Route>
  data: TData
  isLoading: boolean
  children?: React.ReactNode
  index?: number
  mode?: Mode
}

/**
 * This component traverses and renders all components
 * that wrap the selected route (__layout).
 * Parent components must be memoized
 * to prevent re-rendering issues when the route changes.
 */
const TraverseRootComponents = memo(
  ({
    routes,
    data,
    isLoading,
    index = 0,
    mode,
    children,
  }: TraverseRootComponentsProps): React.JSX.Element => {
    if (routes.length > index) {
      const route = routes[index] as Route
      const Parent = route.component

      // Fallback to the route id if the filePath is not defined
      // as is the case for the root route
      const routeFilePath = route.filePath || route.id

      return (
        <Parent data={data} isLoading={isLoading}>
          <CriticalCss routeFilePath={routeFilePath} mode={mode} />
          <TraverseRootComponents
            routes={routes}
            data={data}
            isLoading={isLoading}
            index={index + 1}
            mode={mode}
          >
            {children}
          </TraverseRootComponents>
        </Parent>
      )
    }

    return <>{children}</>
  },
)
TraverseRootComponents.displayName = 'TraverseRootComponents'

const loadParentComponents = (
  route: Route,
  loader: Array<Route> = [],
): Array<Route> => {
  const parentComponent = route.options.getParentRoute?.() as Route

  loader.push(parentComponent)

  if (!parentComponent.isRoot) {
    return loadParentComponents(parentComponent, loader)
  }

  return loader.reverse()
}
