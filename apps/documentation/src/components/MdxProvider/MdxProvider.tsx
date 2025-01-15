import type { JSX, ReactNode } from 'react'
import { MDXProvider } from '@mdx-js/react'

import MdxLink from './MdxLink'
import MdxPre from './MdxPre'
import MdxQuote from './MdxQuote'
import MdxCode from './MdxCode'
import { h } from './MdxTitle'
import MdxBold from './MdxBold'

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
        // @ts-expect-error: useless finding the correct props types
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
      }}
    >
      {children}
    </MDXProvider>
  )
}
