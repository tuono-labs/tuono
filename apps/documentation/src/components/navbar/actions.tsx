import type { JSX } from 'react'
import { Flex, ActionIcon, Group } from '@mantine/core'
import { IconBrandGithub, IconBrandDiscord } from '@tabler/icons-react'

import ThemeBtn from '../theme-btn'

export default function Actions(): JSX.Element {
  return (
    <Flex gap={8}>
      <Group gap={8}>
        <ActionIcon
          variant="default"
          size="lg"
          aria-label="Check the project on github"
          href="https://github.com/tuono-labs/tuono"
          target="_blank"
          component="a"
        >
          <IconBrandGithub />
        </ActionIcon>
        <ActionIcon
          variant="default"
          size="lg"
          aria-label="Join the Tuono's community on Discord!"
          href="https://discord.com/invite/khQzPa654B"
          target="_blank"
          component="a"
        >
          <IconBrandDiscord />
        </ActionIcon>
      </Group>
      <ThemeBtn />
    </Flex>
  )
}
