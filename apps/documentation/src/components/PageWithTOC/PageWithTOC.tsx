import type { JSX, ReactNode } from 'react'
import { Box, Container } from '@mantine/core'

import classes from './PageWithTOC.module.css'

import TableOfContents from '@/components/TableOfContents'

interface PageWithTOCProps {
  children: ReactNode
}

export function PageWithTOC({ children }: PageWithTOCProps): JSX.Element {
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
        py={36}
        className={classes.wrapper}
      >
        {children}
      </Box>

      <TableOfContents />
    </Container>
  )
}
