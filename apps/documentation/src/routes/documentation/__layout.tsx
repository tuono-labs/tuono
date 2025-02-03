import type { JSX, ReactNode } from 'react'
import { Divider } from '@mantine/core'

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

      <Divider mt={20} mb={10} />

      <EditPage />
    </>
  )
}
