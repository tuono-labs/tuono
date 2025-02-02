import type { JSX } from 'react'
import { Title, Text } from '@mantine/core'
import { Link } from 'tuono'

export default function NotFound(): JSX.Element {
  return (
    <>
      <Title order={1} mb={10}>
        404 - Not found
      </Title>

      <Text component="p" mb={10}>
        Sorry, we can’t find the page you’re looking for
      </Text>

      <Link href="/">Back to home</Link>
    </>
  )
}
