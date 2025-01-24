import type { JSX } from 'react'

import type { ServerPayload } from '../types'
import type { Router } from '../router'

import { RouterContextProvider } from './RouterContext'
import { Matches } from './Matches'

interface RouterProviderProps {
  router: Router
  serverPayload?: ServerPayload
}

/**
 * This component is used in the tuono app entry point
 */
export function RouterProvider({
  router,
  serverPayload,
}: RouterProviderProps): JSX.Element {
  return (
    <RouterContextProvider router={router} serverPayload={serverPayload}>
      <Matches serverPayloadData={serverPayload?.data} />
    </RouterContextProvider>
  )
}
