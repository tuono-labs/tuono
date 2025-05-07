import type { JSX } from 'react'
import { RouterProvider } from 'tuono-router'
import type { RouterInstanceType } from 'tuono-router'

import type { Mode } from '../types'

import { useTuonoContextServerPayload } from './TuonoContext'

interface RouterContextProviderWrapperProps {
  router: RouterInstanceType
  mode?: Mode
}

/**
 * This component is needed to get the data from {@link TuonoContext}
 * since the provider is also located in {@link TuonoEntryPoint}
 * hence the context cannot be accessed directly there
 *
 * @see https://github.com/tuono-labs/tuono/issues/410
 */
export function RouterContextProviderWrapper({
  router,
  mode,
}: RouterContextProviderWrapperProps): JSX.Element {
  const serverPayload = useTuonoContextServerPayload()

  return (
    <RouterProvider
      router={router}
      serverInitialLocation={serverPayload.location}
      serverInitialData={serverPayload.data}
      mode={mode}
    />
  )
}
