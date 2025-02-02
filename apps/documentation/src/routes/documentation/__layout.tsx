import type { JSX, ReactNode } from 'react'

import EditPage from '@/components/EditPage'

interface DocumentationLayoutProps {
  children: ReactNode
}

export default function DocumentationLayout({
  children,
}: DocumentationLayoutProps): JSX.Element {
  return (
    <>
      {children}
      <hr />
      <EditPage />
    </>
  )
}
