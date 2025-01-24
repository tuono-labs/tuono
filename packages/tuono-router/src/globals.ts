import type { Router } from './router'
import type { ServerPayload } from './types'

declare global {
  interface Window {
    __TUONO__ROUTER__: Router
    __TUONO_SERVER_PAYLOAD__?: ServerPayload
  }
}
