import type { JSX } from 'react'

import type { ServerInitialLocation } from '../types'
import type { Router } from '../router'

import { RouterContextProvider } from './RouterContext'
import { Matches } from './Matches'

interface RouterProviderProps {
  router: Router
  serverInitialLocation: ServerInitialLocation
  serverInitialData: unknown
}

export function RouterProvider({
  router,
  serverInitialLocation,
  serverInitialData,
}: RouterProviderProps): JSX.Element {
  return (
    <RouterContextProvider
      router={router}
      serverInitialLocation={serverInitialLocation}
    >
      <Matches serverInitialData={serverInitialData} />
    </RouterContextProvider>
  )
}
