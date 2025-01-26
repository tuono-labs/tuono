import type { JSX, HTMLAttributes } from 'react'
import { Code } from '@mantine/core'

export default function MdxCode(
  props: HTMLAttributes<HTMLPreElement>,
): JSX.Element {
  return <Code {...props} style={{ fontSize: 'inherit' }} />
}
