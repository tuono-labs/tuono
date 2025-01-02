import type { ReactNode, JSX } from 'react'

interface RootRouteProps {
  children: ReactNode
}

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  return (
    <html>
      <body>
        <main className="main">{children}</main>
      </body>
    </html>
  )
}
