import type { ElementType, JSX, MouseEventHandler, ReactNode } from 'react'
import { useCallback } from 'react'
import { Title, Anchor, Box } from '@mantine/core'
import type { TitleProps } from '@mantine/core'
import { IconLink } from '@tabler/icons-react'
import { useHover } from '@mantine/hooks'

export default function MdxTitle({
  children,
  order,
  ...rest
}: TitleProps): JSX.Element {
  const headingId = getIdFrom(children)
  const { hovered, ref } = useHover<HTMLHeadingElement>()

  const handleAnchorClick: MouseEventHandler<HTMLAnchorElement> = useCallback(
    (e) => {
      e.preventDefault()
      if (ref.current) {
        history.pushState(
          null,
          '',
          `#${(ref.current.firstChild as HTMLHeadingElement).id} `,
        )
        ref.current.scrollIntoView({
          behavior: 'instant',
          block: 'start',
        })
      }
    },
    [ref],
  )

  return (
    <Box
      ref={ref}
      data-order={order}
      style={{
        scrollMargin: 80,
        marginTop: order === 1 ? 0 : 20,
      }}
      {...rest}
    >
      <Title
        data-heading={headingId}
        id={headingId}
        data-order={order}
        display="inline"
        style={{
          scrollMargin: 80,
          marginTop: order === 1 ? 0 : 20,
        }}
        {...rest}
      >
        {children}
        {hovered && order !== 1 && (
          <Anchor
            onClick={handleAnchorClick}
            display="inline-flex"
            ml={8}
            mb={2}
            style={{ alignItems: 'center', verticalAlign: 'middle' }}
          >
            <IconLink width={20} height={20} />
          </Anchor>
        )}
      </Title>
    </Box>
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
