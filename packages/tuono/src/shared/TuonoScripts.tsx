import type { JSX } from 'react'

import { useRouterContext } from 'tuono-router'

import { DevResources } from './DevResources'
import { ProdResources } from './ProdResources'

export function TuonoScripts(): JSX.Element {
  const { serverSideProps } = useRouterContext()

  return (
    <>
      <script>{`window.__TUONO_SSR_PROPS__=${JSON.stringify(serverSideProps)}`}</script>
      {serverSideProps?.mode === 'Dev' && <DevResources />}
      {serverSideProps?.mode === 'Prod' && <ProdResources />}
    </>
  )
}
