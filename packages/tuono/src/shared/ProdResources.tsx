import type { JSX } from 'react'
import { useRouterContext } from 'tuono-router'

export const ProdResources = (): JSX.Element => {
  const { serverPayload } = useRouterContext()

  return (
    <>
      {serverPayload?.cssBundles?.map((cssHref) => (
        <link
          key={cssHref}
          rel="stylesheet"
          precedence="high"
          type="text/css"
          href={`/${cssHref}`}
        />
      ))}

      {serverPayload?.jsBundles?.map((scriptSrc) => (
        <script key={scriptSrc} type="module" src={`/${scriptSrc}`}></script>
      ))}
    </>
  )
}
