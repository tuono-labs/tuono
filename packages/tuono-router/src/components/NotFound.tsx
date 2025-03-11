import type { JSX } from 'react'

import { useRouterContext } from '../components/RouterContext'
import { ROOT_ROUTE_ID } from '../route'

import { RouteMatch } from './RouteMatch'
import { NotFoundDefaultContent } from './NotFoundDefaultContent'

export function NotFound(): JSX.Element | null {
  const { router } = useRouterContext()

  const custom404Route = router.routesById['/404']

  // Check if exists a custom 404 error page
  if (custom404Route) {
    return <RouteMatch route={custom404Route} serverInitialData={{}} />
  }

  const RootLayout = router.routesById[ROOT_ROUTE_ID]?.component

  if (!RootLayout) return null

  return (
    <RootLayout data={null} isLoading={false}>
      <NotFoundDefaultContent />
    </RootLayout>
  )
}
