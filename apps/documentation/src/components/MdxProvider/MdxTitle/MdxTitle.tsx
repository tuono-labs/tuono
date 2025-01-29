import type { ElementType, JSX, ReactNode } from 'react'
import { Title, type TitleProps } from '@mantine/core'

export default function MdxTitle(props: TitleProps): JSX.Element {
  return (
    <Title
      data-heading={idGen(props.children)}
      data-order={props.order}
      style={{ scrollMargin: 70 }}
      {...props}
      id={idGen(props.children)}
    />
  )
}

function idGen(children: ReactNode): string {
  const getTextContent = (node: ReactNode): string => {
    if (typeof node === 'string') return node
    if (typeof node === 'object' && node !== null && 'props' in node) {
      const child = node as { props?: { children?: ReactNode } }
      return getTextContent(child.props?.children)
    }
    return ''
  }

  if (Array.isArray(children)) {
    const result = children.map(getTextContent).join('')
    return result.toLowerCase().replace(/\s+/g, '-')
  }

  const textContent = getTextContent(children)
  return textContent.toLowerCase().replace(/\s+/g, '-')
}

export const h = (order: 1 | 2 | 3 | 4 | 5 | 6): ElementType<TitleProps> => {
  function render(props: TitleProps): JSX.Element {
    return <MdxTitle order={order} {...props} />
  }
  render.displayName = 'H'
  return render
}
