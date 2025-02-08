import type * as React from 'react'
import { useInView } from 'react-intersection-observer'

import { useRouter } from '../hooks/useRouter'
import { useRoute } from '../hooks/useRoute'

interface TuonoLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  /**
   * If "true" the route gets loaded when the link enters the viewport.
   * @default true
   */
  preload?: boolean

  /**
   * If "false" the scroll offset will be kept across page navigation.
   * @default true
   */
  scroll?: boolean
}

function isModifiedEvent(event: React.MouseEvent): boolean {
  const eventTarget = event.currentTarget as HTMLAnchorElement | SVGAElement
  const target = eventTarget.getAttribute('target')
  return (
    (target && target !== '_self') ||
    event.metaKey ||
    event.ctrlKey ||
    event.shiftKey ||
    event.altKey // triggers resource download
  )
}

export default function Link(
  componentProps: TuonoLinkProps,
): React.JSX.Element {
  const {
    preload = true,
    scroll = true,
    children,
    href,
    onClick,
    ...rest
  } = componentProps

  const router = useRouter()
  const route = useRoute(href)
  const { ref } = useInView({
    onChange(inView) {
      if (inView && preload) route?.component.preload()
    },
    triggerOnce: true,
  })

  const handleTransition: React.MouseEventHandler<HTMLAnchorElement> = (
    event,
  ) => {
    if (href?.startsWith('#') || isModifiedEvent(event)) {
      // If the user is pressing a modifier key, we fall back to default behaviour of `a` tag
      return
    }

    event.preventDefault()
    onClick?.(event)

    router.push(href || '', { scroll })
  }

  return (
    <a {...rest} href={href} ref={ref} onClick={handleTransition}>
      {children}
    </a>
  )
}
