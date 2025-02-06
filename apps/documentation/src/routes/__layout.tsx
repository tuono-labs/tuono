import type { ReactNode, JSX } from 'react'
import { TuonoScripts } from 'tuono'

import {
  ColorSchemeScript,
  createTheme,
  MantineProvider,
  AppShell,
  mantineHtmlProps,
} from '@mantine/core'

import type { CSSVariablesResolver } from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import PostHogProvider from '@/components/PostHogProvider'
import { dynamic } from 'tuono'

import PageWithTOC from '@/components/PageWithTOC'
import Navbar from '@/components/Navbar'
import Sidebar from '@/components/Sidebar'
import MdxProvider from '@/components/MdxProvider'

import '@mantine/core/styles.css'
import '@mantine/code-highlight/styles.css'
import Footer from '@/components/Footer'

const PostHogPageView = dynamic(
  () => import('@/components/PostHogProvider/PostHogPageView'),
  {
    ssr: false,
  },
)

interface RootRouteProps {
  children: ReactNode
}

const theme = createTheme({
  primaryColor: 'violet',
  primaryShade: { light: 6, dark: 9 },
  fontFamily: 'Inter',
  fontFamilyMonospace: 'Menlo',
  respectReducedMotion: true,
  radius: {
    xs: '4px',
    sm: '4px',
    lg: '8px',
    xl: '8px',
    md: '8px',
  },
  fontSizes: {
    // 'xs' | 'sm' | 'md' | 'lg' | 'xl'
    xs: '14px',
    sm: '14px',
  },
  colors: {
    dark: [
      '#d5d7e0',
      '#acaebf',
      '#8c8fa3',
      '#666980',
      '#4d4f66',
      '#34354a',
      '#2b2c3d',
      '#1d1e30',
      '#0c0d21',
      '#01010a',
    ],
  },
  headings: {
    sizes: {
      h1: {
        fontSize: '48px',
      },
    },
  },
  other: {
    sidebarGrayLight: '#495057',
    sidebarGrayDark: '#adb5bd',
    sidebarTextHoverLight: '#212529',
    sidebarTextHoverDark: '#f8f9fa',
  },
})

const resolver: CSSVariablesResolver = (th) => {
  const {
    sidebarGrayLight,
    sidebarTextHoverLight,
    sidebarGrayDark,
    sidebarTextHoverDark,
  } = th.other as Record<string, string>

  return {
    variables: {},
    light: {
      '--mantine-color-footer-bg': th.colors.gray[1],
      '--mantine-color-sidebar-gray': sidebarGrayLight,
      '--mantine-color-sidebar-text-hover': sidebarTextHoverLight,
      '--mantine-color-quote-border': th.colors.violet[1],
    },
    dark: {
      '--mantine-color-footer-bg': th.colors.dark[6],
      '--mantine-color-sidebar-gray': sidebarGrayDark,
      '--mantine-color-sidebar-text-hover': sidebarTextHoverDark,
      '--mantine-color-quote-border': th.colors.violet[9],
    },
  }
}

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  const [opened, { toggle }] = useDisclosure()

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
        <MantineProvider theme={theme} cssVariablesResolver={resolver}>
          <PostHogProvider>
            <PostHogPageView />
            <AppShell
              layout="alt"
              header={{ height: 60 }}
              navbar={{
                width: 300,
                breakpoint: 'sm',
                collapsed: { mobile: !opened },
              }}
            >
              <Navbar toggle={toggle} />
              <Sidebar close={toggle} />
              <AppShell.Main pt={0} px="auto">
                <MdxProvider>
                  <PageWithTOC>{children}</PageWithTOC>
                </MdxProvider>
              </AppShell.Main>
              <Footer />
            </AppShell>
          </PostHogProvider>
        </MantineProvider>
        <TuonoScripts />
      </body>
    </html>
  )
}
