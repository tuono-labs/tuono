import type { Router } from './router'
import type { ServerProps } from './types'

declare global {
  interface Window {
    __TUONO__ROUTER__: Router
    __TUONO_SSR_PROPS__?: ServerProps
  }
}
