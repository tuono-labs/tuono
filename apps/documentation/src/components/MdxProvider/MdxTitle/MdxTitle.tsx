import type { JSX, ReactNode } from 'react'
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
      .map((child: ReactNode) => {
        if (typeof child === 'string') {
          return child;
        }
        if (child && typeof child === 'object') {
          if (child.hasOwnProperty('props') && child.props?.children) {
            return child.props.children;
          }
          return '';
        }

        return '';
      })
      .join('');

    return result
      .toLowerCase()
      .replaceAll(' ', '-');
  }

  // Fallback for non-array children
  return String(children ?? ' ')
    .toLowerCase()
    .replaceAll(' ', '-');
}

export const h = (
  order: 1 | 2 | 3 | 4 | 5 | 6,
): React.ElementType<TitleProps> => {
  function render(props: TitleProps): JSX.Element {
    return <MdxTitle order={order} {...props} />
  }
  render.displayName = 'H'
  return render
}
