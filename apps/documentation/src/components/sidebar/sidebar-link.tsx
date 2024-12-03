import type { JSX, ReactNode } from 'react'
import { NavLink, type NavLinkProps } from '@mantine/core'
import { Link, useRouter } from 'tuono'
import { IconChevronRight } from '@tabler/icons-react'

import styles from './sidebar-link.module.css'

interface SidebarLinkProps {
  label: string
  href: string
  onClick?: () => void
  children?: ReactNode
}

export default function SidebarLink(
  props: SidebarLinkProps & NavLinkProps,
): JSX.Element {
  const { pathname } = useRouter()

  const internalProps = {
    active: pathname === props.href,
    className: styles.link,
    rightSection: props.children && (
      <IconChevronRight
        size="1.2rem"
        stroke={1.5}
        className="mantine-rotate-rtl"
      />
    ),
    autoContrast: true,
    ...props,
  }

  if (props.href.startsWith('#')) {
    return <NavLink component="button" {...internalProps} />
  }

  return <NavLink component={Link} {...internalProps} />
}
