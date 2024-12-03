import type { JSX } from 'react'
import { AppShell, Box, Burger, Button, Flex } from '@mantine/core'
import { Link } from 'tuono'

import Actions from './actions'

interface NavbarProps {
  toggle: () => void
}

export default function Navbar({ toggle }: NavbarProps): JSX.Element {
  return (
    <AppShell.Header p="sm">
      <Flex justify="space-between">
        <Button
          component={Link}
          href="/"
          variant="transparent"
          p={0}
          fz={28}
          hiddenFrom="sm"
        >
          Tuono
        </Button>
        <Box />
        <Flex align="center" gap={8}>
          <Actions />
          <Burger onClick={toggle} hiddenFrom="sm" size="sm" />
        </Flex>
      </Flex>
    </AppShell.Header>
  )
}
