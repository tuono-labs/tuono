import type { JSX, AnchorHTMLAttributes, ReactNode } from 'react'
import { Anchor } from '@mantine/core'
import { Link } from 'tuono'
import { IconExternalLink } from '@tabler/icons-react'

import classes from './MdxLink.module.css'

interface MdxLinkProps extends AnchorHTMLAttributes<HTMLAnchorElement> {
  children: ReactNode
}

export default function MdxLink(props: MdxLinkProps): JSX.Element {
  if (
    props.href?.startsWith('http') ||
    props.href?.startsWith('https') ||
    props.href?.startsWith('mailto')
  ) {
    return (
      <Anchor
        component="a"
        {...props}
        target="_blank"
        variant="transparent"
        className={classes.inner}
        mt={-2}
        p={0}
      >
        {props.children}
        <div className={classes.iconWrapper}>
          <IconExternalLink size="16px" />
        </div>
      </Anchor>
    )
  }
  return (
    <Anchor
      component={Link}
      {...props}
      className={classes.inner}
      target="_blank"
      variant="transparent"
      mt={-2}
      p={0}
    />
  )
}
