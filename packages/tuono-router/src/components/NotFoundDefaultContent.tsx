import type { JSX } from 'react'

import { Link } from './Link'

export function NotFoundDefaultContent(): JSX.Element {
  return (
    <>
      <h1>Page Not Found</h1>
      <Link href="/">Return to Homepage</Link>
    </>
  )
}
