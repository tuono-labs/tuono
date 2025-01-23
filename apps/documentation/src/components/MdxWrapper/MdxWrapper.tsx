import type { JSX } from 'react'
import { Container, Flex, Box, Center } from '@mantine/core'

import TableOfContents from '@/components/TableOfContents'

import EditPage from '../EditPage'
import MdxProvider from '../MdxProvider'

import classes from './MdxWrapper.module.css'

interface MdxWrapperProps {
  children: React.ReactNode
}

export function MdxWrapper({ children }: MdxWrapperProps): JSX.Element {
  return (
    <Center>
      <Flex justify="center">
        <Container
          id="mdx-root"
          component="article"
          size={800}
          p={24}
          className={classes.container}
        >
          <MdxProvider>{children}</MdxProvider>
          <EditPage />
        </Container>
        <Box>
          <TableOfContents
            withTabs={false}
            className={classes.tableOfContents}
          />
        </Box>
      </Flex>
    </Center>
  )
}
