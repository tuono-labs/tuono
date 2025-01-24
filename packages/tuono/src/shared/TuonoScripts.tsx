import type { JSX } from 'react'

import { useRouterContext } from 'tuono-router'

import { DevResources } from './DevResources'
import { ProdResources } from './ProdResources'

export function TuonoScripts(): JSX.Element {
  const { serverPayload } = useRouterContext()

  return (
    <>
      <script>{`window.__TUONO_SERVER_PAYLOAD__=${JSON.stringify(serverPayload)}`}</script>
      {serverPayload?.mode === 'Dev' && <DevResources />}
      {serverPayload?.mode === 'Prod' && <ProdResources />}
    </>
  )
}
