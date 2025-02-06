import type { JSX, ReactNode } from 'react'
import { MDXProvider } from '@mdx-js/react'
import { Divider } from '@mantine/core'

import MdxLink from './MdxLink'
import MdxPre from './MdxPre'
import MdxQuote from './MdxQuote'
import MdxCode from './MdxCode'
import { h } from './MdxTitle'
import MdxBold from './MdxBold'
import MdxTable from './MdxTable'
import MdxUl from './MdxUl'
import MdxParagraph from './MdxParagraph'

interface MdxProviderProps {
  children: ReactNode
}

export default function MdxProvider({
  children,
}: MdxProviderProps): JSX.Element {
  return (
    <MDXProvider
      components={{
        a: MdxLink,
        pre: MdxPre,
        blockquote: MdxQuote,
        code: MdxCode,
        h1: h(1),
        h2: h(2),
        h3: h(3),
        h4: h(4),
        h5: h(5),
        h6: h(6),
        strong: MdxBold,
        table: MdxTable,
        thead: MdxTable.Thead,
        tbody: MdxTable.Tbody,
        tr: MdxTable.Tr,
        th: MdxTable.Th,
        td: MdxTable.Td,
        p: MdxParagraph,
        ul: MdxUl,
        hr: Divider,
      }}
    >
      {children}
    </MDXProvider>
  )
}
