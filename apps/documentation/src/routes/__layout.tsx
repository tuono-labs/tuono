import type { ReactNode, JSX } from 'react'
import { TuonoScripts } from 'tuono'

import { ColorSchemeScript, mantineHtmlProps } from '@mantine/core'

import { PostHogProvider, PostHogPageView } from '@/components/PostHog'

import '@mantine/core/styles.css'
import '@mantine/code-highlight/styles.css'

import App from '@/components/App'

interface RootRouteProps {
  children: ReactNode
}

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  return (
    <html lang="en" {...mantineHtmlProps}>
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/apple-touch-icon.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/favicon-32x32.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="16x16"
          href="/favicon-16x16.png"
        />
        <link rel="manifest" href="/site.webmanifest" />
        <ColorSchemeScript />
      </head>
      <body>
        <PostHogProvider>
          <PostHogPageView />
          <App>{children}</App>
        </PostHogProvider>
        <TuonoScripts />
      </body>
    </html>
  )
}
