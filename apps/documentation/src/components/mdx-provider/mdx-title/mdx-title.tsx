import type { JSX } from 'react'
import { Title, type TitleProps } from '@mantine/core'

export default function MdxTitle(props: TitleProps): JSX.Element {
  return (
    <Title
      data-heading={props.children}
      data-order={props.order}
      mt={20}
      {...props}
      // eslint-disable-next-line @typescript-eslint/no-base-to-string
      id={String(props.children ?? '')
        .toLowerCase()
        .replaceAll(' ', '-')}
    />
  )
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
