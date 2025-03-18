import type { ReactNode, JSX } from 'react'
import { TuonoScripts } from 'tuono'

interface RootLayoutProps {
  children: ReactNode
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
  return (
    <html>
      <body>
        <main>{children}</main>
        <TuonoScripts />
      </body>
    </html>
  )
}
