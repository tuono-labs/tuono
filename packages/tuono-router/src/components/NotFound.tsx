import type { JSX } from 'react'

import { useRouterContext } from '../components/RouterContext'
import { ROOT_ROUTE_ID } from '../route'

import type { Mode } from '../types'

import { RouteMatch } from './RouteMatch'
import { NotFoundDefaultContent } from './NotFoundDefaultContent'
import { CriticalCss } from './CriticalCss'

export function NotFound({ mode }: { mode?: Mode }): JSX.Element | null {
  const { router } = useRouterContext()

  const custom404Route = router.routesById['/404']

  // Check if exists a custom 404 error page
  if (custom404Route) {
    return (
      <>
        <CriticalCss routeFilePath={custom404Route.filePath} mode={mode} />
        <RouteMatch route={custom404Route} mode={mode} serverInitialData={{}} />
      </>
    )
  }

  const RootLayout = router.routesById[ROOT_ROUTE_ID]?.component

  if (!RootLayout) return null

  return (
    <RootLayout data={null} isLoading={false}>
      <CriticalCss routeFilePath="__root__" mode={mode} />
      <NotFoundDefaultContent />
    </RootLayout>
  )
}
