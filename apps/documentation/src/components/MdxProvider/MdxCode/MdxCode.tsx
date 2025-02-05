import type { JSX, HTMLAttributes } from 'react'
import { Code } from '@mantine/core'

export default function MdxCode(
  props: HTMLAttributes<HTMLPreElement>,
): JSX.Element {
  console.log(props)
  return <Code {...props} style={{ fontSize: 'inherit' }} />
}
