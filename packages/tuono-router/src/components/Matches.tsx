import type * as React from 'react'

import useRoute from '../hooks/useRoute'

import { RouteMatch } from './RouteMatch'
import NotFound from './NotFound'
import { useInternalRouter } from './RouterContext'

interface MatchesProps<TServerSideProps = unknown> {
  // user defined props
  serverSideProps: TServerSideProps
}

export function Matches({ serverSideProps }: MatchesProps): React.JSX.Element {
  const { location } = useInternalRouter()

  const route = useRoute(location.pathname)

  if (!route) {
    return <NotFound />
  }

  return <RouteMatch route={route} serverSideProps={serverSideProps} />
}
