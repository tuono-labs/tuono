import type { JSX, ReactNode } from 'react'
import { Container, Box } from '@mantine/core'

import TableOfContents from '@/components/TableOfContents'

import EditPage from '../EditPage'
import MdxProvider from '../MdxProvider'

import classes from './MdxWrapper.module.css'

interface MdxWrapperProps {
  children: ReactNode
}

export function MdxWrapper({ children }: MdxWrapperProps): JSX.Element {
  return (
    <Container size={1000} className={classes.wrapper}>
      <Box
        id="mdx-root"
        component="article"
        p={24}
        className={classes.container}
      >
        <MdxProvider>{children}</MdxProvider>
        <EditPage />
      </Box>
      <Box>
        <TableOfContents withTabs={false} className={classes.tableOfContents} />
      </Box>
    </Container>
  )
}
