import type { ReactNode, JSX } from 'react'

import {
  ColorSchemeScript,
  createTheme,
  MantineProvider,
  AppShell,
  Container,
} from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { Head } from 'tuono'

import EditPage from '@/components/edit-page'
import MdxProvider from '@/components/mdx-provider'
import Navbar from '@/components/navbar'
import Sidebar from '@/components/sidebar'

import '@mantine/core/styles.css'
import '@mantine/code-highlight/styles.css'

interface RootRouteProps {
  children: ReactNode
}

const theme = createTheme({
  primaryColor: 'violet',
  primaryShade: { light: 6, dark: 9 },
  fontFamily: 'Roboto',
  respectReducedMotion: true,
  radius: {
    xs: '8px',
    lg: '8px',
    xl: '8px',
    md: '8px',
    sm: '8px',
  },
  fontSizes: {
    // 'xs' | 'sm' | 'md' | 'lg' | 'xl'
    xs: '16px',
    sm: '16px',
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
})

export default function RootRoute({ children }: RootRouteProps): JSX.Element {
  const [opened, { toggle }] = useDisclosure()

  return (
    <>
      <Head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link
          rel="icon"
          href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>⚡</text></svg>"
        />
        <link rel="manifest" href="/site.webmanifest" />
      </Head>
      <ColorSchemeScript />
      <MantineProvider theme={theme}>
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
          <AppShell.Main>
            <Container id="mdx-root" component="article" size="md" p={20}>
              <MdxProvider>{children}</MdxProvider>
              <EditPage />
            </Container>
          </AppShell.Main>
        </AppShell>
      </MantineProvider>
    </>
  )
}
