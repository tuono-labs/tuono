import type { ServerPayload } from './types'

declare global {
  interface Window {
    __TUONO_SERVER_PAYLOAD__?: ServerPayload
  }
}
