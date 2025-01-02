import type { ReactNode, JSX } from 'react'
import { MDXProvider } from '@mdx-js/react'

interface RootRouteProps {
  children: ReactNode
}

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  return (
    <html>
      <body>
        <main className="main">
          <MDXProvider components={{}}>{children}</MDXProvider>
        </main>
      </body>
    </html>
  )
}
