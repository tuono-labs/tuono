import type { ReactNode, JSX } from 'react'
import { MDXProvider } from '@mdx-js/react'
import { TuonoScripts } from 'tuono'
import '../styles/global.css'

interface RootLayoutProps {
  children: ReactNode
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
  return (
    <html>
      <body>
        <main>
          <MDXProvider components={{}}>{children}</MDXProvider>
        </main>
        <TuonoScripts />
      </body>
    </html>
  )
}
