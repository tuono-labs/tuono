import type { JSX } from 'react'
import { AppShell, Badge, Flex, Title, Button } from '@mantine/core'

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
      <SidebarLink href="/" label="Home" onClick={close} />
      <SidebarLink
        href="/documentation/installation"
        label="Installation"
        onClick={close}
      />
      <SidebarLink
        href="/documentation/tutorial"
        label="Tutorial"
        defaultOpened
        onClick={close}
      >
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
      <SidebarLink href="/documentation/cli" label="CLI" onClick={close} />
      <SidebarLink
        label="Application state"
        href="/documentation/application-state"
        onClick={close}
      />

      <SidebarLink
        label="Routing"
        href="/documentation/routing"
        onClick={close}
      >
        <SidebarLink
          href="/documentation/routing/intro"
          label="Project structure"
          onClick={close}
        />
      </SidebarLink>

      <SidebarLink
        label="Contributing"
        href="/documentation/contributing"
        leftSection="✨"
        onClick={close}
      >
        <SidebarLink
          href="/documentation/contributing/local-development"
          label="Local development"
          onClick={close}
        />
      </SidebarLink>
    </AppShell.Navbar>
  )
}
