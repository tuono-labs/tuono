import { type JSX, StrictMode } from 'react'
import { type RouterType, type ServerProps, RouterProvider } from 'tuono-router'

interface TuonoEntryPointProps {
  router: RouterType
  serverProps?: ServerProps
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
