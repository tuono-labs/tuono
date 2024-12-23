import React from 'react'
import type { ReactNode, JSX } from 'react'

import { useListenBrowserUrlUpdates } from '../hooks/useListenBrowserUrlUpdates'
import { useInitRouterStore } from '../hooks/useRouterStore'
import type { ServerProps } from '../types'
import type { Router } from '../router'

import { getRouterContext } from './RouterContext'
import { Matches } from './Matches'

interface RouterContextProviderProps {
  router: Router
  children: ReactNode
}

function RouterContextProvider({
  router,
  children,
}: RouterContextProviderProps): JSX.Element {
  // Allow the router to update options on the router instance
  router.update({ ...router.options } as Parameters<typeof router.update>[0])

  const routerContext = getRouterContext()

  return (
    <React.Suspense>
      <routerContext.Provider value={router}>{children}</routerContext.Provider>
    </React.Suspense>
  )
}

interface RouterProviderProps {
  router: Router
  serverProps?: ServerProps
}

export function RouterProvider({
  router,
  serverProps,
}: RouterProviderProps): JSX.Element {
  useInitRouterStore(serverProps)

  useListenBrowserUrlUpdates()

  return (
    <RouterContextProvider router={router}>
      <Matches serverSideProps={serverProps?.props} />
    </RouterContextProvider>
  )
}
