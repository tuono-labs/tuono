import type { JSX } from 'react'
import { Container } from '@mantine/core'

import EditPage from '../EditPage'
import MdxProvider from '../MdxProvider'

import classes from './MdxWrapper.module.css'

import TableOfContents from '@/components/TableOfContents'

interface MdxWrapperProps {
  children: React.ReactNode
}

export function MdxWrapper({ children }: MdxWrapperProps): JSX.Element {
  return (
    <div className={classes.wrapper}>
      <Container
        id="mdx-root"
        component="article"
        size="md"
        p={20}
        className={classes.container}
      >
        <MdxProvider>{children}</MdxProvider>
        <EditPage />
      </Container>
      <TableOfContents withTabs={false} className={classes.tableOfContents} />
    </div>
  )
}
