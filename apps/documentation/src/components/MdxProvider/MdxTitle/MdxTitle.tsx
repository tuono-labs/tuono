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

  return textContent.toLowerCase().replace(/[\s\W_]+/g, '-')
}

export const h = (order: 1 | 2 | 3 | 4 | 5 | 6): ElementType<TitleProps> => {
  function render(props: TitleProps): JSX.Element {
    return <MdxTitle order={order} {...props} />
  }
  render.displayName = 'H'

  return render
}
