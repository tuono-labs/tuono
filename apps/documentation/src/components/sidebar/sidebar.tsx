import type { JSX } from 'react'
import { AppShell } from '@mantine/core'

import SidebarLink from './sidebar-link'

interface SidebarProps {
  close: () => void
}

export default function Sidebar({ close }: SidebarProps): JSX.Element {
  return (
    <AppShell.Navbar p="md">
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
