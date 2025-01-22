/*
 * Component inspired by: https://github.com/mantinedev/mantine/tree/master/apps/mantine.dev/src/components/TableOfContents
 */
import type { JSX } from 'react'
import { useEffect, useRef, useState } from 'react'
import { useRouter } from 'tuono'
import { IconList } from '@tabler/icons-react'
import { Box, rem, ScrollArea, Text } from '@mantine/core'

import { getHeadings, type Heading } from './getHeadings'
import classes from './TableOfContents.module.css'

function getActiveElement(rects: Array<DOMRect>): number {
  if (rects.length === 0) {
    return -1
  }

  const closest = rects.reduce(
    (acc, item, index) => {
      if (Math.abs(acc.position) < Math.abs(item.y)) {
        return acc
      }

      return {
        index,
        position: item.y,
      }
    },
    { index: 0, position: rects[0].y },
  )

  return closest.index
}

interface TableOfContentsProps {
  withTabs: boolean
  className?: string
}

export function TableOfContents({
  withTabs,
  className,
}: TableOfContentsProps): JSX.Element | null {
  const [active, setActive] = useState(0)
  const [headings, setHeadings] = useState<Array<Heading>>([])
  const headingsRef = useRef<Array<Heading>>([])
  const router = useRouter()

  const filteredHeadings = headings.filter((heading) => heading.depth > 1)

  const handleScroll = (): void => {
    setActive(
      getActiveElement(
        headingsRef.current.map((d) => d.getNode().getBoundingClientRect()),
      ),
    )
  }

  useEffect(() => {
    const _headings = getHeadings()
    headingsRef.current = _headings
    setHeadings(_headings)
    setActive(
      getActiveElement(
        _headings.map((d) => d.getNode().getBoundingClientRect()),
      ),
    )
    window.addEventListener('scroll', handleScroll)
    return (): void => {
      window.removeEventListener('scroll', handleScroll)
    }
  }, [router.pathname])

  if (filteredHeadings.length === 0) {
    return null
  }

  const items = filteredHeadings.map((heading, index) => (
    <Text
      key={heading.id}
      component="a"
      fz="sm"
      p={10}
      className={classes.link}
      truncate
      mod={{ active: active === index }}
      href={`#${heading.id}`}
      __vars={{ '--toc-link-offset': `${heading.depth - 1}` }}
    >
      {heading.content}
    </Text>
  ))

  return (
    <Box
      component="nav"
      mod={{ 'with-tabs': withTabs }}
      className={classes.wrapper + ' ' + className}
    >
      <div className={classes.inner}>
        <div>
          <div className={classes.header}>
            <IconList
              style={{ width: rem(20), height: rem(20) }}
              stroke={1.5}
            />
            <Text className={classes.title}>Table of contents</Text>
          </div>
          <ScrollArea.Autosize
            mah={`calc(100vh - ${rem(140)})`}
            style={{ overflow: 'hidden' }}
            type="never"
            offsetScrollbars
          >
            <div className={classes.items}>{items}</div>
          </ScrollArea.Autosize>
        </div>
      </div>
    </Box>
  )
}
