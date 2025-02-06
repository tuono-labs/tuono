import type { JSX } from 'react'
import { Link } from 'tuono'
import {
  Container,
  Box,
  AppShell,
  Title,
  Divider,
  Flex,
  Text,
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
    <Box mb={24}>
      <Flex gap={12} align="center" h={size} mb={8}>
        <img src="/logo.svg" alt="Tuono" width={size} height={size} />
        <Title order={2} fz={size}>
          Tuono
        </Title>
      </Flex>
      <Text fz={12}>Simply the fastest full-stack web framework ⚡</Text>
    </Box>
  )
}

function FooterOutro(): JSX.Element {
  return (
    <Container
      size={1000}
      w="100%"
      display="flex"
      style={{ gap: 12, justifyContent: 'space-between' }}
    >
      <Box>
        <Text fz={14}>
          Built by{' '}
          <Anchor fz={14} href="https://github.com/Valerioageno">
            Valerio Ageno
          </Anchor>{' '}
          and{' '}
          <Anchor
            fz={14}
            href="https://github.com/tuono-labs/tuono/graphs/contributors"
          >
            these awesome people
          </Anchor>
        </Text>
      </Box>
    </Container>
  )
}

export default function Footer(): JSX.Element {
  return (
    <AppShell.Footer id="app-footer" withBorder={false} zIndex={0} p={0}>
      <Box m={0} pt={52} pb={8} w="100%" bg="var(--mantine-color-footer-bg)">
        <Container
          size={1000}
          w="100%"
          display="flex"
          style={{ gap: 12, justifyContent: 'space-between' }}
          mb={24}
        >
          <Flex
            justify="space-between"
            w="100%"
            direction={{ base: 'column', xs: 'row' }}
          >
            <FooterTitle />
            <Flex
              gap={{ base: 12, md: 32 }}
              justify="space-between"
              direction={{ base: 'column', xs: 'row' }}
            >
              <FooterLinkList title="About" links={aboutLinks} />
              <FooterLinkList title="Community" links={communityLinks} />
            </Flex>
          </Flex>
          <Box
            w={220}
            miw={220}
            id="table-of-content-placeholder"
            visibleFrom="lg"
          />
        </Container>
        <Divider mb={8} mt={28} />
        <FooterOutro />
      </Box>
    </AppShell.Footer>
  )
}
