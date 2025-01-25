import type { JSX, MouseEvent } from 'react'
import { useRef, useState, useEffect } from 'react'
import { useRouter } from 'tuono'
import { Box, Text } from '@mantine/core'

import { getHeadings, type Heading } from './getHeadings'
import classes from './TableOfContents.module.css'

interface TableOfContentsProps {}

export function TableOfContents({}: TableOfContentsProps): JSX.Element | null {
  const [active, setActive] = useState<number | null>(null)
  const [headings, setHeadings] = useState<Array<Heading>>([])
  const headingsRef = useRef<Array<HTMLElement>>([])
  const observerRef = useRef<IntersectionObserver | null>(null)
  const router = useRouter()

  useEffect(() => {
    const _headings = getHeadings()
    setHeadings(_headings)
    headingsRef.current = _headings.map((heading) => heading.getNode())

    if (observerRef.current) {
      observerRef.current.disconnect()
    }

    const observer = new IntersectionObserver(
      (entries) => {
        const visibleEntries = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top)

        if (visibleEntries.length > 0) {
          setActive(
            _headings.findIndex((h) => h.id === visibleEntries[0].target.id),
          )
        }
      },
      {
        rootMargin: '-50px 0px -80% 0px',
        threshold: [0.1, 0.5, 1.0],
      },
    )

    headingsRef.current.forEach((node) => {
      observer.observe(node)
    })
    observerRef.current = observer

    const handleHashChange = (): void => {
      setTimeout(() => {
        observerRef.current?.disconnect()
        observerRef.current = observer
        headingsRef.current.forEach((node) => {
          observer.observe(node)
        })
      }, 300)
    }

    window.addEventListener('hashchange', handleHashChange)

    return (): void => {
      observer.disconnect()
      window.removeEventListener('hashchange', handleHashChange)
    }
  }, [router.pathname])

  const handleClick = (
    event: MouseEvent<HTMLAnchorElement>,
    id: string,
  ): void => {
    event.preventDefault()
    const element = document.getElementById(id)
    if (element) {
      element.scrollIntoView({
        behavior: 'instant',
        block: 'start',
      })
    }
  }

  // Avoid to show it in case of a TODO page
  if (headings.length === 1) {
    return null
  }

  return (
    <Box component="nav" className={classes.wrapper}>
      <div className={classes.inner}>
        <div>
          <Text className={classes.title} mb={8}>
            On this page
          </Text>
          <div className={classes.items}>
            {headings.slice(1).map((heading, index) => (
              <Text
                key={heading.id}
                component="a"
                fz="sm"
                px={8}
                w="fit-content"
                py={4}
                className={classes.link}
                mod={{ active: active === index + 1 }}
                href={`#${heading.id}`}
                onClick={(e) => {
                  handleClick(e, heading.id)
                }}
                __vars={{ '--toc-link-offset': `${heading.depth - 1}` }}
              >
                {heading.content}
              </Text>
            ))}
          </div>
        </div>
      </div>
    </Box>
  )
}
