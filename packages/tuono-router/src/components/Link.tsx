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

  /**
   * If "true" the history entry will be replaced instead of pushed.
   * @default false
   */
  replace?: boolean
}

function isEventModifierKeyActiveAndTargetDifferentFromSelf(
  event: React.MouseEvent<HTMLAnchorElement>,
): boolean {
  const target = event.currentTarget.getAttribute('target')
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
    replace,
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
    onClick?.(event)

    if (
      href?.startsWith('#') ||
      // If the user is pressing a modifier key or using the target attribute,
      // we fall back to default behaviour of `a` tag
      isEventModifierKeyActiveAndTargetDifferentFromSelf(event)
    ) {
      return
    }

    event.preventDefault()

    router[replace ? 'replace' : 'push'](href || '', { scroll })
  }

  return (
    <a {...rest} href={href} ref={ref} onClick={handleTransition}>
      {children}
    </a>
  )
}
