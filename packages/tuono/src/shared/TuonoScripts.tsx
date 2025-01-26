import type { JSX } from 'react'

import { SERVER_PAYLOAD_VARIABLE_NAME } from '../constants'

import { DevResources } from './DevResources'
import { ProdResources } from './ProdResources'
import { useTuonoContextServerPayload } from './TuonoContext'

export function TuonoScripts(): JSX.Element {
  const serverPayload = useTuonoContextServerPayload()

  return (
    <>
      <script>{`window['${SERVER_PAYLOAD_VARIABLE_NAME}']=${JSON.stringify(serverPayload)}`}</script>
      {serverPayload.mode === 'Dev' && <DevResources />}
      {serverPayload.mode === 'Prod' && <ProdResources />}
    </>
  )
}
