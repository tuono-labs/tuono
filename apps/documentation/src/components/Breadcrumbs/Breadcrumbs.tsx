import { useMemo, type JSX } from 'react'
import { Breadcrumbs as MantineBreadcrumbs, Button } from '@mantine/core'
import { Link } from 'tuono'

import { IconChevronRight, IconBolt } from '@tabler/icons-react'

interface BreadcrumbData {
  href?: string
  label: string
}
interface BreadcrumbsProps {
  breadcrumbs: Array<BreadcrumbData>
}

export default function Breadcrumbs({
  breadcrumbs = [],
}: BreadcrumbsProps): JSX.Element {
  const ldJson = useMemo(() => {
    const _ldJson = {
      '@context': 'https://schema.org',
      '@type': 'BreadcrumbList',
      itemListElement: [
        {
          '@type': 'ListItem',
          position: 1,
          name: 'Tuono - The React/Rust fullstack framework',
          item: 'https://tuono.dev' as string | undefined,
        },
      ],
    }

    _ldJson.itemListElement.push(
      ...breadcrumbs.map((br, i) => ({
        '@type': 'ListItem' as const,
        position: i + 3,
        name: br.label,
        item: br.href ? `https://tuono.dev${br.href}` : undefined,
      })),
    )

    return _ldJson
  }, [breadcrumbs])

  return (
    <>
      <script type="application/ld+json">
        {JSON.stringify(ldJson, null, 2)}
      </script>
      <MantineBreadcrumbs
        separator={<IconChevronRight size="1.1rem" stroke={1.5} />}
        mb="md"
        mt="md"
      >
        <Button href="/" component={Link} variant="subtle" radius="xl" p={5}>
          <IconBolt />
        </Button>
        {breadcrumbs.map((br) => (
          <BreadcrumbElement href={br.href} label={br.label} key={br.label} />
        ))}
      </MantineBreadcrumbs>
    </>
  )
}

interface BreadcrumbElementProps {
  href?: string
  label: string
}

export function BreadcrumbElement({
  href,
  label,
}: BreadcrumbElementProps): JSX.Element {
  if (href) {
    return (
      <Button component={Link} href={href} variant="subtle" radius="xl" px={10}>
        {label}
      </Button>
    )
  }

  return (
    <Button component="span" variant="light" radius="xl">
      {label}
    </Button>
  )
}
