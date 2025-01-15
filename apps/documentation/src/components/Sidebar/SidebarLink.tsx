import type { JSX, ReactNode } from 'react'
import { NavLink, type NavLinkProps } from '@mantine/core'
import clsx from 'clsx'
import { Link, useRouter } from 'tuono'
import { IconChevronRight } from '@tabler/icons-react'

import styles from './SidebarLink.module.css'

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

  const isActive = pathname === props.href

  const internalProps = {
    active: isActive,
    className: clsx(styles.link, isActive && styles.active),
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
