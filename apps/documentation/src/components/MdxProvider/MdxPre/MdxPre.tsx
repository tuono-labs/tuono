import type { JSX } from 'react'
import { CodeHighlight } from '@mantine/code-highlight'

import styles from './MdxPre.module.css'

interface PreProps {
  children: {
    props: {
      children: string
      className?: string
    }
  }
}

export default function MdxPre({ children }: PreProps): JSX.Element {
  return (
    <CodeHighlight
      className={styles.pre}
      code={children.props.children || ''}
      style={{ borderRadius: 8, fontWeight: 100 }}
      language={children.props.className?.replace('language-', '')}
    />
  )
}
