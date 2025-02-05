import type { JSX } from 'react'
import { Link } from 'tuono'
import {
  Container,
  Box,
  AppShell,
  Title,
  Divider,
  Flex,
  Anchor,
} from '@mantine/core'

const aboutLinks: Array<Link> = [
  { title: 'Contribute', url: '/documentation/contributing' },
  {
    title: 'GitHub releases',
    url: 'https://github.com/tuono-labs/tuono/releases',
  },
]

const communityLinks: Array<Link> = [
  { title: 'Chat on Discord', url: 'https://discord.com/invite/khQzPa654B' },
  { title: 'Follow on X', url: 'https://twitter.com/valerioageno' },
  { title: 'Follow on GitHub', url: 'https://github.com/tuono-labs' },
]

interface Link {
  title: string
  url: string
}

interface FooterLinkListProps {
  title: string
  links: Array<Link>
}

function FooterLink({ link }: { link: Link }): JSX.Element {
  const component = link.url.startsWith('http') ? 'a' : Link
  const target = link.url.startsWith('http') ? '_blank' : undefined
  return (
    <Box>
      <Anchor
        component={component}
        href={link.url}
        key={link.url}
        fz={14}
        target={target}
      >
        {link.title}
      </Anchor>
    </Box>
  )
}

function FooterLinkList({ title, links }: FooterLinkListProps): JSX.Element {
  return (
    <Box>
      <Title order={5} mb={8}>
        {title}
      </Title>
      {links.map((link) => (
        <FooterLink link={link} />
      ))}
    </Box>
  )
}

function FooterTitle(): JSX.Element {
  const size = 44
  return (
    <Flex gap={12} align="center" h={size} mb={24}>
      <img src="/logo.svg" alt="Tuono" width={size} height={size} />
      <Title order={2} fz={size}>
        Tuono
      </Title>
    </Flex>
  )
}

export default function Footer(): JSX.Element {
  return (
    <AppShell.Footer id="app-footer" withBorder={false} zIndex={0} p={0}>
      <Box m={0} pt={36} pb={8} w="100%" bg="var(--mantine-color-footer-bg)">
        <Container size="1000">
          <Flex
            justify="space-between"
            direction={{ base: 'column', xs: 'row' }}
          >
            <FooterTitle />
            <Flex
              gap={{ base: 12, xs: 20 }}
              justify="space-between"
              direction={{ base: 'column', xs: 'row' }}
            >
              <FooterLinkList title="About" links={aboutLinks} />
              <FooterLinkList title="Community" links={communityLinks} />
            </Flex>
          </Flex>
          <Divider mb={8} mt={16} />
          <Box>
            <span>© 2025 Tuono</span>
          </Box>
        </Container>
      </Box>
    </AppShell.Footer>
  )
}
