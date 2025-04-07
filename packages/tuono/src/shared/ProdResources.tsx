import type { JSX } from 'react'

interface ProdResourcesProps {
  jsBundles: Array<string> | null
  cssBundles: Array<string> | null
}

export const ProdResources = ({
  cssBundles,
  jsBundles,
}: ProdResourcesProps): JSX.Element => {
  return (
    <>
      {cssBundles?.map((cssHref) => (
        <link
          key={cssHref}
          rel="stylesheet"
          precedence="high"
          type="text/css"
          href={`/${cssHref}`}
        />
      ))}

      {jsBundles?.map((scriptSrc) => (
        <script key={scriptSrc} type="module" src={`/${scriptSrc}`}></script>
      ))}
    </>
  )
}
