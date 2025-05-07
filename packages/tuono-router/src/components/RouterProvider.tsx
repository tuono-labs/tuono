import type { JSX } from 'react'

import type { ServerInitialLocation, Mode } from '../types'
import type { Router } from '../router'

import { RouterContextProvider } from './RouterContext'
import { Matches } from './Matches'

interface RouterProviderProps {
  router: Router
  serverInitialLocation: ServerInitialLocation
  serverInitialData: unknown
  mode?: Mode
}

export function RouterProvider({
  router,
  serverInitialLocation,
  serverInitialData,
  mode,
}: RouterProviderProps): JSX.Element {
  return (
    <RouterContextProvider
      router={router}
      serverInitialLocation={serverInitialLocation}
    >
      <Matches serverInitialData={serverInitialData} mode={mode} />
    </RouterContextProvider>
  )
}
