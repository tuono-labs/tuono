import type { ReactNode, JSX } from 'react'
import { TuonoScripts } from 'tuono'

interface RootLayoutProps {
  children: ReactNode
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
  return (
    <html>
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body>
        <main>{children}</main>
        <TuonoScripts />
      </body>
    </html>
  )
}
