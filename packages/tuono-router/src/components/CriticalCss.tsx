import type { JSX } from 'react'

import type { Mode } from '../types'

const VITE_PROXY_PATH = '/vite-server'
const CRITICAL_CSS_PATH = VITE_PROXY_PATH + '/tuono_internal__critical_css'

interface CriticalCssProps {
  routeFilePath?: string
  mode?: Mode
}

/**
 * Returns the critical CSS for the given route
 * This is required in order to avoid FOUC during development
 * since vite does not support CSS injection without JS waterfall
 */
export function CriticalCss({
  routeFilePath,
  mode,
}: CriticalCssProps): JSX.Element | null {
  if (!routeFilePath || mode !== 'Dev') {
    return null
  }

  return (
    <link
      href={`${CRITICAL_CSS_PATH}?componentId=${routeFilePath}`}
      precedence="high"
      rel="stylesheet"
    />
  )
}
