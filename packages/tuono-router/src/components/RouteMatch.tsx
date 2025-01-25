import * as React from 'react'

import type { Route } from '../route'
import { useServerPayloadData } from '../hooks/useServerPayloadData'

interface RouteMatchProps<TServerPayloadData = unknown> {
  route: Route
  // User defined server side props
  serverInitialData: TServerPayloadData
}

/**
 * Returns the route match with the root element if exists
 *
 * It handles the fetch of the client side resources
 */
export const RouteMatch = ({
  route,
  serverInitialData: serverInitialData,
}: RouteMatchProps): React.JSX.Element => {
  const { data, isLoading } = useServerPayloadData(route, serverInitialData)

  // eslint-disable-next-line react-hooks/exhaustive-deps
  const routes = React.useMemo(() => loadParentComponents(route), [route.id])

  return (
    <TraverseRootComponents routes={routes} data={data} isLoading={isLoading}>
      <React.Suspense>
        <route.component data={data} isLoading={isLoading} />
      </React.Suspense>
    </TraverseRootComponents>
  )
}

interface TraverseRootComponentsProps<TData = unknown> {
  routes: Array<Route>
  data: TData
  isLoading: boolean
  children?: React.ReactNode
  index?: number
}

/*
 * This component traverses and renders
 * all the components that wraps the selected route (__layout).
 * The parents components need to be memoized in order to avoid
 * re-rendering bugs when changing route.
 */
const TraverseRootComponents = React.memo(
  ({
    routes,
    data,
    isLoading,
    index = 0,
    children,
  }: TraverseRootComponentsProps): React.JSX.Element => {
    if (routes.length > index) {
      const Parent = (routes[index] as Route).component

      return (
        <Parent data={data} isLoading={isLoading}>
          <TraverseRootComponents
            routes={routes}
            data={data}
            isLoading={isLoading}
            index={index + 1}
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
