import { StrictMode } from 'react'
import type { JSX } from 'react'
import type { RouterInstanceType } from 'tuono-router'

import type { ServerPayload } from '../types'

import { TuonoContextProvider } from './TuonoContext'
import { RouterContextProviderWrapper } from './RouterContextProviderWrapper'
import { ErrorOverlay } from './ErrorOverlay'

interface TuonoEntryPointProps {
  router: RouterInstanceType
  serverPayload?: ServerPayload
}

export function TuonoEntryPoint({
  router,
  serverPayload,
}: TuonoEntryPointProps): JSX.Element {
  return (
    <StrictMode>
      <TuonoContextProvider serverPayload={serverPayload}>
        <ErrorOverlay>
          <RouterContextProviderWrapper router={router} />
        </ErrorOverlay>
      </TuonoContextProvider>
    </StrictMode>
  )
}
