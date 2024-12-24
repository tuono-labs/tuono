import { type ReactNode, type JSX, Suspense } from 'react'

interface RootRouteProps {
  children: ReactNode
}

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  return (
    <html>
      <body className="main">{children}</body>
    </html>
  )
}
