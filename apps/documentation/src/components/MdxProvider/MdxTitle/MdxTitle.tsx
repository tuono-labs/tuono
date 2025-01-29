import type { ElementType, JSX, ReactNode } from 'react'
import { Title, type TitleProps } from '@mantine/core'

export default function MdxTitle(props: TitleProps): JSX.Element {
  const headingId = getIdFrom(props.children)

  return (
    <Title
      data-heading={headingId}
      data-order={props.order}
      style={{ scrollMargin: 70 }}
      {...props}
      id={headingId}
    />
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
