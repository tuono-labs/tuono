import type { JSX } from 'react'

import { DevResources } from './DevResources'
import { ProdResources } from './ProdResources'
import { useTuonoContextServerPayload } from './TuonoContext'

export function TuonoScripts(): JSX.Element {
  const serverPayload = useTuonoContextServerPayload()

  return (
    <>
      <script>{`window.__TUONO_SERVER_PAYLOAD__=${JSON.stringify(serverPayload)}`}</script>
      {serverPayload.mode === 'Dev' && <DevResources />}
      {serverPayload.mode === 'Prod' && <ProdResources />}
    </>
  )
}
