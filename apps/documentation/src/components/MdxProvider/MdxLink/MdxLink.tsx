import type { JSX, AnchorHTMLAttributes, ReactNode } from 'react'
import { Anchor } from '@mantine/core'
import { Link } from 'tuono'
import { IconExternalLink } from '@tabler/icons-react'

interface MdxLinkProps extends AnchorHTMLAttributes<HTMLAnchorElement> {
  children: ReactNode
}

export default function MdxLink(props: MdxLinkProps): JSX.Element {
  if (props.href?.startsWith('http') || props.href?.startsWith('mailto')) {
    return (
      <Anchor
        {...props}
        target="_blank"
        variant="transparent"
        display="inline"
        style={{ fontWeight: 600 }}
        mt={-2}
        p={0}
      >
        {props.children}
        <IconExternalLink
          size="16px"
          style={{
            marginLeft: '4px',
            display: 'inline-block',
            transform: 'translateY(2px)',
          }}
        />
      </Anchor>
    )
  }
  return (
    <Anchor
      component={Link}
      {...props}
      target="_blank"
      variant="transparent"
      display="inline"
      style={{ fontWeight: 600 }}
      mt={-2}
      p={0}
    />
  )
}
