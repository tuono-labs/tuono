import type { JSX, HTMLAttributes } from 'react'
import { Text } from '@mantine/core'

import styles from './MdxText.module.css'

export default function MdxText(
  props: HTMLAttributes<HTMLParagraphElement>,
): JSX.Element {
  return (
    <Text
      {...props}
      className={styles.text}
      inherit
      size="md"
      my={20}
      fw={300}
    />
  )
}
