import type { JSX, ReactNode } from 'react'
import { Box, Container } from '@mantine/core'

import TableOfContents from '@/components/TableOfContents'

import EditPage from '../EditPage'
import MdxProvider from '../MdxProvider'

import classes from './MdxWrapper.module.css'

interface MdxWrapperProps {
  children: ReactNode
}

export function MdxWrapper({ children }: MdxWrapperProps): JSX.Element {
  return (
    <Container
      size={1000}
      w="100%"
      display="flex"
      style={{ gap: 12, justifyContent: 'space-between' }}
    >
      <Box
        id="mdx-root"
        component="article"
        mt="xl"
        px={16}
        py={36}
        className={classes.wrapper}
      >
        <MdxProvider>{children}</MdxProvider>
        <EditPage />
      </Box>
      <Box>
        <TableOfContents />
      </Box>
    </Container>
  )
}
