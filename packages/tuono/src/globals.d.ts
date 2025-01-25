import type { ServerPayload } from './types'
import type { SERVER_PAYLOAD_VARIABLE_NAME } from './constants'

declare global {
  interface Window {
    [SERVER_PAYLOAD_VARIABLE_NAME]?: ServerPayload
  }
}
