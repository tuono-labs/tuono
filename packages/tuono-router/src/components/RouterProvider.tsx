import type { JSX } from 'react'
import { StrictMode } from 'react'

import type { ServerProps } from '../types'
import type { Router } from '../router'

import { RouterContextProvider } from './RouterContext'
import { Matches } from './Matches'

interface RouterProviderProps {
  router: Router
  serverProps?: ServerProps
}

/**
 * This component is used in the tuono app entry point
 */
export function RouterProvider({
  router,
  serverProps,
}: RouterProviderProps): JSX.Element {
  return (
    <StrictMode>
      <RouterContextProvider router={router} serverSideProps={serverProps}>
        <Matches serverSideProps={serverProps?.props} />
      </RouterContextProvider>
    </StrictMode>
  )
}
