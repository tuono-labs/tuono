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
        href: '/documentation/installation',
      },
      {
        type: 'element',
        label: 'How is tuono different?',
        href: '/documentation/how-is-tuono-different',
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
        label: 'Defining routes',
        href: '/documentation/routing/defining-routes',
      },
      {
        type: 'element',
        label: 'Link and navigation',
        href: '/documentation/routing/link-and-navigation',
      },
      {
        type: 'element',
        label: 'Pages',
        href: '/documentation/routing/pages',
      },
      {
        type: 'element',
        label: 'Loading state',
        href: '/documentation/routing/loading-state',
      },
      {
        type: 'element',
        label: 'Dynamic routes',
        href: '/documentation/routing/dynamic-routes',
      },
      {
        type: 'element',
        label: 'Layouts',
        href: '/documentation/routing/layouts',
      },
      {
        type: 'element',
        label: 'Redirecting',
        href: '/documentation/routing/redirecting',
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
        label: 'Server side rendering',
        href: '/documentation/rendering/server-side-rendering',
      },
      {
        type: 'element',
        label: 'Static site rendering',
        href: '/documentation/rendering/static-site-rendering',
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
        label: 'Head',
        href: '/documentation/components/head',
      },
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
    label: 'tuono.config.ts',
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
    ],
  },
]
