import type { JSX } from 'react'
import { MDXProvider } from '@mdx-js/react'
import { TuonoScripts } from 'tuono'
import type { TuonoLayoutProps } from 'tuono'

import '../styles/global.css'

export default function RootLayout({
  children,
}: TuonoLayoutProps): JSX.Element {
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
