import { StrictMode } from 'react'
import type { JSX } from 'react'
import type { RouterInstanceType } from 'tuono-router'

import type { ServerPayload } from '../types'

import { TuonoContextProvider } from './TuonoContext'
import { RouterContextProviderWrapper } from './RouterContextProviderWrapper'

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
        <RouterContextProviderWrapper
          router={router}
          mode={serverPayload?.mode}
        />
      </TuonoContextProvider>
    </StrictMode>
  )
}
