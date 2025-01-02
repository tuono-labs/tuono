import type { ReactNode } from 'react'

import type { Mode } from '../types'

interface ProductionBunldesProps {
  bundles: Array<string>
  mode: Mode
}

export const ProductionCssLinks = ({
  bundles,
  mode,
}: ProductionBunldesProps): ReactNode => {
  if (mode === 'Dev') return null
  return (
    <>
      {bundles.map((cssHref) => (
        <link
          key={cssHref}
          rel="stylesheet"
          type="text/css"
          href={`/${cssHref}`}
        />
      ))}
    </>
  )
}

export const ProductionScriptLinks = ({
  bundles,
  mode,
}: ProductionBunldesProps): ReactNode => {
  if (mode === 'Dev') return null
  return (
    <>
      {bundles.map((scriptSrc) => (
        <script key={scriptSrc} type="module" src={`/${scriptSrc}`}></script>
      ))}
    </>
  )
}
