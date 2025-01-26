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
    event.preventDefault()
    onClick?.(event)

    if (href?.startsWith('#')) {
      window.location.hash = href
      return
    }

    router.push(href || '', { scroll })
  }

  return (
    <a {...rest} href={href} ref={ref} onClick={handleTransition}>
      {children}
    </a>
  )
}
