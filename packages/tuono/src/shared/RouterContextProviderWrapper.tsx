import type { JSX } from 'react'
import { RouterProvider } from 'tuono-router'
import type { RouterInstanceType } from 'tuono-router'

import { useTuonoContextServerPayload } from './TuonoContext'

interface RouterContextProviderWrapperProps {
  router: RouterInstanceType
}

/**
 * This component is needed to get the data from {@link TuonoContext}
 * since the provider is also located in {@link TuonoEntryPoint}
 * hence the context cannot be accessed directly there
 *
 * @see https://github.com/tuono-labs/tuono/issues/410
 */
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
