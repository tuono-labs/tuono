import type * as React from 'react'

import { useRoute } from '../hooks/useRoute'

import { RouteMatch } from './RouteMatch'
import NotFound from './NotFound'
import { useRouterContext } from './RouterContext'

interface MatchesProps<TServerPayloadData = unknown> {
  // user defined props
  serverInitialData: TServerPayloadData
}

export function Matches({
  serverInitialData,
}: MatchesProps): React.JSX.Element {
  const { location } = useRouterContext()

  const route = useRoute(location.pathname)

  if (!route) {
    return <NotFound />
  }

  return <RouteMatch route={route} serverInitialData={serverInitialData} />
}
