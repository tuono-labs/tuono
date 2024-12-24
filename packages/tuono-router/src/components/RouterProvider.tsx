import { Suspense, type JSX } from 'react'

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
    <RouterContextProvider
      router={router}
      serverSideProps={serverProps?.router}
    >
      <Matches serverSideProps={serverProps?.props} />
    </RouterContextProvider>
  )
}
