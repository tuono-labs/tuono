import type { ElementType, JSX, ReactNode } from 'react'
import { Title, type TitleProps } from '@mantine/core'

export default function MdxTitle(props: TitleProps): JSX.Element {
  return (
    <Title
      data-heading={props.children}
      data-order={props.order}
      mt={20}
      {...props}
      id={idGen(props.children)}
    />
  )
}

function idGen(children: ReactNode): string {
  if (Array.isArray(children)) {
    const result = children
      .map((child) => {
        if (typeof child === 'string') {
          return child
        }
        if (typeof child === 'object' && child !== null && 'props' in child) {
          const childWithProps = child as { props?: { children?: ReactNode } }
          return typeof childWithProps.props?.children === 'string'
            ? childWithProps.props.children
            : ''
        }
        return ''
      })
      .join('')

    return result.toLowerCase().replace(/\s+/g, '-')
  }

  return typeof children === 'string'
    ? children.toLowerCase().replace(/\s+/g, '-')
    : ''
}

export const h = (order: 1 | 2 | 3 | 4 | 5 | 6): ElementType<TitleProps> => {
  function render(props: TitleProps): JSX.Element {
    return <MdxTitle order={order} {...props} />
  }
  render.displayName = 'H'
  return render
}
