import type { JSX } from 'react'

interface ProdResourcesProps {
  cssBundles: Array<string>
  jsBundles: Array<string>
}

export const ProdResources = ({
  cssBundles,
  jsBundles,
}: ProdResourcesProps): JSX.Element => (
  <>
    {cssBundles.map((cssHref) => (
      <link
        key={cssHref}
        rel="stylesheet"
        type="text/css"
        href={`/${cssHref}`}
      />
    ))}

    {jsBundles.map((scriptSrc) => (
      <script key={scriptSrc} type="module" src={`/${scriptSrc}`}></script>
    ))}
  </>
)
