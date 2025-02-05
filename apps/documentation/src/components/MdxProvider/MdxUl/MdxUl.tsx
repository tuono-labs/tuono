import type { HTMLAttributes, JSX } from 'react'
import { List } from '@mantine/core'
import styles from './MdxUl.module.css'

export default function MdxUl(
  props: HTMLAttributes<HTMLUListElement>,
): JSX.Element {
  return <List {...props} className={styles.list} pl={16} my={12} />
}
