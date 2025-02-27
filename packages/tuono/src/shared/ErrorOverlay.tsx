import { useEffect } from 'react'
import type { JSX, ReactNode } from 'react'
import type { ErrorPayload } from 'vite'
import { useTuonoContextServerPayload } from './TuonoContext'

const showErrorOverlay = (err: Partial<ErrorPayload['err']>): void => {
  // must be within function call because that's when the element is defined for sure.
  const ErrorOverlay = customElements.get('vite-error-overlay')
  // don't open outside vite environment
  if (!ErrorOverlay) {
    return
  }
  const overlay = new ErrorOverlay(err)
  const tip = overlay.shadowRoot?.querySelector('.tip')
  if (tip) {
    // The default `tip` includes informations about vite.config.ts
    // hence we are replacing it with a more generic message.
    tip.innerHTML =
      'Click outside, press <kbd>Esc</kbd> key, or fix the code to dismiss.'
  }
  document.body.appendChild(overlay)
}

interface ErrorOverlayProps {
  children: ReactNode
}

/**
 * Display the vite overlay error when a runtime error occurs.
 *
 * @see https://github.com/vitejs/vite/issues/2076
 */
export function ErrorOverlay({ children }: ErrorOverlayProps): JSX.Element {
  const { mode } = useTuonoContextServerPayload()
  useEffect(() => {
    if (mode === 'Prod') {
      // Never display the error overlay in production
      return
    }

    window.addEventListener('error', ({ error }) => {
      showErrorOverlay(error as ErrorPayload['err'])
    })
    window.addEventListener('unhandledrejection', ({ reason }) => {
      showErrorOverlay(reason as ErrorPayload['err'])
    })

    return (): void => {
      window.removeEventListener('error', ({ error }) => {
        showErrorOverlay(error as ErrorPayload['err'])
      })
      window.removeEventListener('unhandledrejection', ({ reason }) => {
        showErrorOverlay(reason as ErrorPayload['err'])
      })
    }
  }, [])

  return <>{children}</>
}
