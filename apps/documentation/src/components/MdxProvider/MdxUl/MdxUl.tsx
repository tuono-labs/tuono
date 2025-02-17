import type { HTMLAttributes, JSX } from 'react'
import { List } from '@mantine/core'

import styles from './MdxUl.module.css'

export default function MdxUl(
  props: HTMLAttributes<HTMLUListElement>,
): JSX.Element {
  return (
    <List
      {...props}
      className={styles.list}
      my={12}
      size="md"
      style={{ listStylePosition: 'outside' }}
      fw={300}
      withPadding
    />
  )
}
