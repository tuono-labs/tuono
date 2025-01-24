import type * as React from 'react'

import { useRouterContext } from '../components/RouterContext'

import { RouteMatch } from './RouteMatch'
import Link from './Link'

export default function NotFound(): React.JSX.Element {
  const { router } = useRouterContext()

  const custom404Route = router.routesById['/404']

  // Check if exists a custom 404 error page
  if (custom404Route) {
    return <RouteMatch route={custom404Route} serverInitialData={{}} />
  }

  return (
    <>
      <h1>404 Not found</h1>
      <Link href="/">Return home</Link>
    </>
  )
}
