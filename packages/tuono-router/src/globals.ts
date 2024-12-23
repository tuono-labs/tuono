import type React from 'react'

import type { Router } from './router'

declare global {
  interface Window {
    __TUONO__ROUTER__: Router
    __TUONO_SSR_PROPS__?: {
      props?: unknown
    }

    __TUONO_CONTEXT__?: React.Context<Router>
  }
}
