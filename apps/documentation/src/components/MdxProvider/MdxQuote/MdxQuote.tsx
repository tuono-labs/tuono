import type { JSX, HTMLAttributes } from 'react'
import { Blockquote, Space } from '@mantine/core'

export default function MdxQuote(
  props: HTMLAttributes<HTMLQuoteElement>,
): JSX.Element {
  return (
    <>
      <Blockquote
        color="violet"
        py={1}
        px={20}
        mt={30}
        iconSize={30}
        {...props}
        style={{ borderRadius: 8 }}
      />
      <Space h="md" />
    </>
  )
}
