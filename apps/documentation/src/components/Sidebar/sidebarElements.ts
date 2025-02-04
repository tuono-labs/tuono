import type { ReactNode } from 'react'

interface SidebarElementBase<T extends string> {
  type: T
}

type SidebarDivider = SidebarElementBase<'divider'>

interface SidebarLink extends SidebarElementBase<'element'> {
  label: string
  href: string
  children?: Array<SidebarLink>
  leftIcon?: ReactNode
}

interface SidebarTitle extends SidebarElementBase<'title'> {
  label: string
}

type SidebarElement = SidebarDivider | SidebarLink | SidebarTitle

export const sidebarElements: Array<SidebarElement> = [
  {
    type: 'element',
    label: 'Home',
    href: '/',
  },
  {
    type: 'element',
    label: 'Getting started',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Installation',
        href: '/documentation/getting-started/installation',
      },
      {
        type: 'element',
        label: 'Setup project',
        href: '/documentation/getting-started/setup-project',
      },
    ],
  },
  {
    type: 'element',
    label: 'Tutorial',
    href: '#focus',
    children: [
      {
        type: 'element',
        href: '/documentation/tutorial',
        label: 'Intro',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/development-setup',
        label: 'Development setup',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/api-fetching',
        label: 'API fetching',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/components',
        label: 'Components',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/dynamic-routes',
        label: 'Dynamic routes',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/error-handling',
        label: 'Error handling',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/seo',
        label: 'SEO and meta tags',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/redirections',
        label: 'Redirections',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/production',
        label: 'Production build',
      },
      {
        type: 'element',
        href: '/documentation/tutorial/conclusion',
        label: 'Conclusion',
      },
    ],
  },
  { type: 'divider' },
  { type: 'title', label: 'Overview' },
  {
    type: 'element',
    label: 'Routing',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Pages and layout',
        href: '/documentation/routing/defining-routes',
      },
      {
        type: 'element',
        label: 'Dynamic routes',
        href: '/documentation/routing/dynamic-routes',
      },
      {
        type: 'element',
        label: 'Link and navigation',
        href: '/documentation/routing/link-and-navigation',
      },
    ],
  },
  {
    type: 'element',
    label: 'Rendering',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Server side rendering (SSR)',
        href: '/documentation/rendering/server-side-rendering',
      },
      {
        type: 'element',
        label: 'Static site generation (SSG)',
        href: '/documentation/rendering/static-site-generation',
      },
    ],
  },
  {
    type: 'element',
    label: 'Styles',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'CSS modules',
        href: '/documentation/styles/css-modules',
      },
      {
        type: 'element',
        label: 'Tailwind CSS',
        href: '/documentation/styles/tailwind-css',
      },
    ],
  },
  {
    type: 'element',
    label: 'Integrations',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'MDX',
        href: '/documentation/integrations/mdx',
      },
    ],
  },
  {
    type: 'element',
    label: 'Core concepts',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Multithreading',
        href: '/documentation/core-concepts/multithreading',
      },
    ],
  },
  { type: 'divider' },
  { type: 'title', label: 'API reference' },
  {
    type: 'element',
    label: 'Components',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Link',
        href: '/documentation/components/link',
      },
    ],
  },
  {
    type: 'element',
    label: 'Hooks',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'useRouter',
        href: '/documentation/hooks/use-router',
      },
    ],
  },
  {
    type: 'element',
    label: 'Server utilities',
    href: '#focus',
    children: [
      {
        type: 'element',
        label: 'Response',
        href: '/documentation/server-utilities/response',
      },
    ],
  },
  {
    type: 'element',
    label: 'CLI',
    href: '/documentation/cli',
  },
  {
    type: 'element',
    label: 'Application state',
    href: '/documentation/application-state',
  },
  {
    type: 'element',
    label: 'Configuration',
    href: '/documentation/configuration',
  },

  { type: 'divider' },
  {
    type: 'element',
    label: 'Contributing',
    href: '#focus',
    leftIcon: '✨',
    children: [
      {
        type: 'element',
        label: 'Guildelines',
        href: '/documentation/contributing',
      },
      {
        type: 'element',
        label: 'Local development',
        href: '/documentation/contributing/local-development',
      },
      {
        type: 'element',
        label: 'Pull requests',
        href: '/documentation/contributing/pull-requests',
      },
    ],
  },
]
