import type { JSX, ReactNode } from 'react'
import { useEffect, useRef, useState } from 'react'
import { Box, Container } from '@mantine/core'
import { useViewportSize } from '@mantine/hooks'

import TableOfContents from '@/components/TableOfContents'

import classes from './PageWithTOC.module.css'

interface PageWithTOCProps {
  children: ReactNode
}

export function PageWithTOC({ children }: PageWithTOCProps): JSX.Element {
  const { width } = useViewportSize()
  const [footerHeight, setFooterHeight] = useState<number>(0)
  const mainRef = useRef<HTMLDivElement>(null)
  const height = `calc(100% - ${footerHeight}px)`

  useEffect(() => {
    const footer = document.getElementById('app-footer')

    if (footer) {
      setFooterHeight(footer.clientHeight)
    }
  }, [
    setFooterHeight,
    // Reload on window width resize
    width,
  ])

  return (
    <>
      <Box
        style={{ zIndex: 10, height }}
        pos="relative"
        ref={mainRef}
        mb={footerHeight}
        bg="var(--mantine-color-body)"
      >
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
      </Box>
    </>
  )
}
