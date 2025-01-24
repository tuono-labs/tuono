import type { JSX } from 'react'
import { RouterProvider } from 'tuono-router'
import type { RouterInstanceType } from 'tuono-router'

import { useTuonoContextServerPayload } from './TuonoContext'

interface RouterContextProviderWrapperProps {
  router: RouterInstanceType
}

export function RouterContextProviderWrapper(
  props: RouterContextProviderWrapperProps,
): JSX.Element {
  const { router } = props

  const serverPayload = useTuonoContextServerPayload()

  return (
    <RouterProvider
      router={router}
      serverInitialLocation={serverPayload.location}
      serverInitialData={serverPayload.data}
    />
  )
}
