import type { JSX, ReactNode } from 'react'
import { Box, Container } from '@mantine/core'

import TableOfContents from '@/components/TableOfContents'

import EditPage from '../EditPage'
import MdxProvider from '../MdxProvider'

interface MdxWrapperProps {
  children: ReactNode
}

export function MdxWrapper({ children }: MdxWrapperProps): JSX.Element {
  return (
    <Container size={1000} w="100%" display="flex" style={{ gap: 12 }}>
      <Box
        id="mdx-root"
        component="article"
        mt="xl"
        px={16}
        py={36}
        style={{ maxWidth: '100%' }}
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
