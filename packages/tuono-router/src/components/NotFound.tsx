import type * as React from 'react'

import { useInternalRouter } from '../components/RouterContext'

import { RouteMatch } from './RouteMatch'
import Link from './Link'

export default function NotFound(): React.JSX.Element {
  const { router } = useInternalRouter()

  const custom404Route = router.routesById['/404']

  // Check if exists a custom 404 error page
  if (custom404Route) {
    return <RouteMatch route={custom404Route} serverSideProps={{}} />
  }

  return (
    <>
      <h1>404 Not found</h1>
      <Link href="/">Return home</Link>
    </>
  )
}
