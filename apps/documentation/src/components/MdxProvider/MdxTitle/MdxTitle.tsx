import {
  useCallback,
  type ElementType,
  type JSX,
  type MouseEvent,
  type ReactNode,
} from 'react'
import { Title, Anchor, type TitleProps } from '@mantine/core'
import { IconLink } from '@tabler/icons-react'
import { useHover } from '@mantine/hooks'

export default function MdxTitle(props: TitleProps): JSX.Element {
  const headingId = getIdFrom(props.children)
  const { hovered, ref } = useHover<HTMLHeadingElement>()

  const onLinkClick = useCallback(
    (e: MouseEvent<HTMLAnchorElement>): void => {
      e.preventDefault()
      if (ref.current) {
        ref.current.scrollIntoView({
          behavior: 'instant',
          block: 'start',
        })
      }
    },
    [ref],
  )

  return (
    <Title
      ref={ref}
      data-heading={headingId}
      data-order={props.order}
      style={{
        scrollMargin: 70,
        marginTop: props.order === 1 ? 0 : 20,
        display: 'flex',
        alignItems: 'center',
        gap: 8,
      }}
      {...props}
      id={headingId}
    >
      {props.children}
      {hovered && props.order !== 1 && (
        <Anchor onClick={onLinkClick} h={20}>
          <IconLink width={20} height={20} />
        </Anchor>
      )}
    </Title>
  )
}

function getIdFrom(children: ReactNode): string {
  const getTextContent = (node: ReactNode): string => {
    if (typeof node === 'string') return node
    if (typeof node === 'object' && node !== null && 'props' in node) {
      const child = node as { props?: { children?: ReactNode } }
      return getTextContent(child.props?.children)
    }
    return ''
  }

  const textContent = Array.isArray(children)
    ? children.map(getTextContent).join('')
    : getTextContent(children)

  return (
    textContent
      // normalize cause tuono build --static to hang
      // @see https://github.com/tuono-labs/tuono/issues/468
      // .normalize('NFKD')// separate accented characters into their base form and diacritical marks
      .replace(/[\u0300-\u036f]/g, '') // remove all the accents
      .trim()
      .toLowerCase()
      .replace(/\./g, '-') // some titles (configuration) contain keypath, so replace dots with hyphens
      .replace(/[^a-z0-9 -]/g, '') // remove non-alphanumeric characters
      .replace(/\s+/g, '-') // replace spaces with hyphens
      .replace(/-+/g, '-') // remove consecutive hyphens
  )
}

export const h = (order: 1 | 2 | 3 | 4 | 5 | 6): ElementType<TitleProps> => {
  function render(props: TitleProps): JSX.Element {
    return <MdxTitle order={order} {...props} />
  }
  render.displayName = 'H'

  return render
}
