import { type JSX, StrictMode } from 'react'
import { type RouterInstanceType, RouterProvider } from 'tuono-router'

import type { ServerPayload } from '../types'

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
      <RouterProvider router={router} serverPayload={serverPayload} />
    </StrictMode>
  )
}
