import { type JSX, StrictMode } from 'react'
import { type createRouter, RouterProvider } from 'tuono-router'

interface TuonoEntryPointProps {
  router: ReturnType<typeof createRouter>
  serverProps?: never
}

export function TuonoEntryPoint({
  router,
  serverProps,
}: TuonoEntryPointProps): JSX.Element {
  return (
    <StrictMode>
      <RouterProvider router={router} serverProps={serverProps} />
    </StrictMode>
  )
}
