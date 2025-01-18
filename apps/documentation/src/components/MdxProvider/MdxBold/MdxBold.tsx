import type { JSX } from 'react'
import { Text, type TextProps } from '@mantine/core'

export default function MdxBold(props: TextProps): JSX.Element {
  return <Text component="span" fw={700} {...props} />
}
