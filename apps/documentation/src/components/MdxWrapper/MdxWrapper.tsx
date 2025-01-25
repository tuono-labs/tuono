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
    <Container size={1000} display="flex">
      <Box id="mdx-root" component="article" p={14} mt="xl">
        <MdxProvider>{children}</MdxProvider>
        <EditPage />
      </Box>
      <Box>
        <TableOfContents />
      </Box>
    </Container>
  )
}
