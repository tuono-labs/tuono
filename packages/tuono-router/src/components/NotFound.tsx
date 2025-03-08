import type { JSX } from 'react'

import { useRouterContext } from '../components/RouterContext'

import { RouteMatch } from './RouteMatch'
import { Link } from './Link'

export function NotFound(): JSX.Element | null {
  const { router } = useRouterContext()

  const custom404Route = router.routesById['/404']

  // Check if exists a custom 404 error page
  if (custom404Route) {
    return <RouteMatch route={custom404Route} serverInitialData={{}} />
  }

  const RootLayout = router.routesById['__root__']?.component

  if (!RootLayout) return null

  return (
    <RootLayout data={{}} isLoading={false}>
      <h1>404 Not found</h1>
      <Link href="/">Return home</Link>
    </RootLayout>
  )
}
