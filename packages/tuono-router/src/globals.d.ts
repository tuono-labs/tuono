import type { Router } from './router'

declare global {
  interface Window {
    __TUONO__ROUTER__: Router
  }
}
