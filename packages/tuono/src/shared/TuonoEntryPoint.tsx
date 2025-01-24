import { type JSX, StrictMode } from 'react'
import { type RouterInstanceType, RouterProvider } from 'tuono-router'

import type { ServerPayload } from '../types'

import { TuonoContextProvider } from './TuonoContext'

interface TuonoEntryPointProps {
  router: RouterInstanceType
  serverPayload?: ServerPayload
}

export function TuonoEntryPoint({
  router,
  serverPayload,
}: TuonoEntryPointProps): JSX.Element {
  return (
    <StrictMode>
      <TuonoContextProvider serverPayload={serverPayload}>
        <RouterProvider router={router} serverPayload={serverPayload} />
      </TuonoContextProvider>
    </StrictMode>
  )
}
