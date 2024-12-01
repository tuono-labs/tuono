import type { JSX } from 'react'
import { Breadcrumbs, Button } from '@mantine/core'
import { Link, Head } from 'tuono'

import { IconChevronRight, IconBolt } from '@tabler/icons-react'

interface Breadcrumb {
  href?: string
  label: string
}
interface BreadcrumbsProps {
  breadcrumbs: Breadcrumb[]
}

export default function TuonoBreadcrumbs({
  breadcrumbs = [],
}: BreadcrumbsProps): JSX.Element {
  return (
    <>
      <Head>
        <script type="application/ld+json">
          {`{
              "@context": "https://schema.org",
              "@type": "BreadcrumbList",
              "itemListElement": [
				{
                  "@type": "ListItem",
                  "position": 1,
                  "name": "Tuono - The React/Rust fullstack framework",
                  "item": "https://tuono.dev"
                }${breadcrumbs.length > 0 ? ',' : ''}
                ${breadcrumbs
                  .map((br, i) =>
                    JSON.stringify({
                      '@type': 'ListItem',
                      position: i + 3,
                      name: br.label,
                      item: br.href ? `https://tuono.dev${br.href}` : undefined,
                    }),
                  )
                  .join(',')}]
            }
          `}
        </script>
      </Head>
      <Breadcrumbs
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
      </Breadcrumbs>
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
