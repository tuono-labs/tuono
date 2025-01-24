import type { JSX } from 'react'

import { useTuonoContextServerPayload } from './TuonoContext'

export const ProdResources = (): JSX.Element => {
  const serverPayload = useTuonoContextServerPayload()

  return (
    <>
      {serverPayload.cssBundles?.map((cssHref) => (
        <link
          key={cssHref}
          rel="stylesheet"
          precedence="high"
          type="text/css"
          href={`/${cssHref}`}
        />
      ))}

      {serverPayload.jsBundles?.map((scriptSrc) => (
        <script key={scriptSrc} type="module" src={`/${scriptSrc}`}></script>
      ))}
    </>
  )
}
