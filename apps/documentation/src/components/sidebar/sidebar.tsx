import type { JSX } from 'react'

import {
  AppShell,
  Badge,
  Flex,
  Divider,
  Title,
  Button,
  ScrollArea,
  Text,
} from '@mantine/core'

import { IconX } from '@tabler/icons-react'
import { useMediaQuery } from '@mantine/hooks'

import { sidebarElements } from './config'
import SidebarLink from './sidebar-link'

interface SidebarProps {
  close: () => void
}

function SidebarHeader({ close }: SidebarProps): JSX.Element {
  const isSm = useMediaQuery('(min-width: 48em)')
  return (
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
  )
}

function SidebarElements({ close }: SidebarProps): JSX.Element {
  return (
    <AppShell.Section component={ScrollArea}>
      {sidebarElements.map((el, i) => {
        if (el.type === 'divider') {
          return <Divider key={`${el.type}-${i}`} my="md" mx={10} />
        }

        if (el.type === 'title') {
          return (
            <Text
              key={`${el.type}-${i}`}
              size="xs"
              fw={700}
              fz={12}
              pl={12}
              py={5}
            >
              {el.label}
            </Text>
          )
        }

        if (el.children?.length) {
          return (
            <SidebarLink
              key={`${el.type}-${i}`}
              href={el.href}
              label={el.label}
              leftSection={el.leftIcon}
            >
              {el.children.map((child, index) => (
                <SidebarLink
                  href={child.href}
                  label={child.label}
                  key={index}
                  onClick={close}
                />
              ))}
            </SidebarLink>
          )
        }

        return (
          <SidebarLink
            key={`${el.type}-${i}`}
            href={el.href}
            label={el.label}
            onClick={close}
          />
        )
      })}
    </AppShell.Section>
  )
}

export default function Sidebar({ close }: SidebarProps): JSX.Element {
  return (
    <AppShell.Navbar p="md">
      <SidebarHeader close={close} />
      <SidebarElements close={close} />
    </AppShell.Navbar>
  )
}
