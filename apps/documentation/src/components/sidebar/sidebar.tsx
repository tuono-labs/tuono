import type { JSX } from 'react'
import {
  AppShell,
  Badge,
  Flex,
  Divider,
  Title,
  Button,
  ScrollArea,
} from '@mantine/core'

import SidebarLink from './sidebar-link'
import { IconX } from '@tabler/icons-react'
import { useMediaQuery } from '@mantine/hooks'

interface SidebarProps {
  close: () => void
}

export default function Sidebar({ close }: SidebarProps): JSX.Element {
  const isSm = useMediaQuery('(min-width: 48em)')
  return (
    <AppShell.Navbar p="md">
      <AppShell.Section>
        <Flex mb={20} w="100%" justify="space-between">
          <Flex
            gap={10}
            align="center"
            w={isSm ? '100%' : 'auto'}
            justify="center"
          >
            <Title order={3}>Tuono</Title>
            <Badge mt={4} size="xs" variant="outline">
              Docs
            </Badge>
          </Flex>
          <Button onClick={close} hiddenFrom="sm" variant="transparent" p="0">
            <IconX />
          </Button>
        </Flex>
      </AppShell.Section>
      <AppShell.Section component={ScrollArea}>
        <SidebarLink href="/" label="Home" onClick={close} />
        <SidebarLink href="#required-for-focus" label="Getting started">
          <SidebarLink
            href="/installation"
            label="Installation"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/development-setup"
            label="Development setup"
            onClick={close}
          />
          <SidebarLink href="/documentation/cli" label="CLI" onClick={close} />
        </SidebarLink>
        <SidebarLink href="#required-for-focus" label="Tutorial">
          <SidebarLink
            href="/documentation/tutorial"
            label="Tutorial"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/development-setup"
            label="Development setup"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/api-fetching"
            label="API fetching"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/components"
            label="Components"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/dynamic-routes"
            label="Dynamic routes"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/error-handling"
            label="Error handling"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/seo"
            label="SEO and meta tags"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/redirections"
            label="Server redirection"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/production"
            label="Production build"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/tutorial/conclusion"
            label="Conclusion"
            onClick={close}
          />
        </SidebarLink>
        <Divider my="md" />
        <SidebarLink label="Routing" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="Project structure"
            onClick={close}
          />
          <SidebarLink
            label="Application state"
            href="/documentation/application-state"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink label="Rendering" href="#required-for-focus">
          <SidebarLink
            label="Server side rendering"
            href="/documentation/routing"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/routing/intro"
            label="Static rendering"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink label="Styles" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="CSS modules"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink label="Integrations" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="MDX"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink label="Core concepts" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="Multithreading"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink
          label="Contributing"
          href="#required-for-focus"
          leftSection="✨"
        >
          <SidebarLink
            label="Guildelines"
            href="/documentation/contributing"
            onClick={close}
          />
          <SidebarLink
            href="/documentation/contributing/local-development"
            label="Local development"
            onClick={close}
          />
        </SidebarLink>
        <Divider my="md" />
        <SidebarLink label="Components" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="Head"
            onClick={close}
          />
        </SidebarLink>
        <SidebarLink label="Hooks" href="#required-for-focus">
          <SidebarLink
            href="/documentation/routing/intro"
            label="Head"
            onClick={close}
          />
        </SidebarLink>
      </AppShell.Section>
    </AppShell.Navbar>
  )
}
