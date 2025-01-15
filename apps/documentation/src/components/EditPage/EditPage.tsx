import type { JSX } from 'react'
import { Button } from '@mantine/core'
import { IconEdit } from '@tabler/icons-react'
import { useRouter } from 'tuono'

const GITHUB_URL =
  'https://github.com/tuono-labs/tuono/tree/main/apps/documentation/src/routes'

export default function EditPage(): JSX.Element {
  const { pathname } = useRouter()
  return (
    <Button
      p={0}
      mt={60}
      component="a"
      variant="transparent"
      leftSection={<IconEdit />}
      target="_blank"
      href={GITHUB_URL.concat(pathname).concat('.mdx')}
    >
      Edit page
    </Button>
  )
}
