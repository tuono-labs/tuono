import type { JSX } from 'react'
import { useRouterContext } from 'tuono-router'

const VITE_PROXY_PATH = '/vite-server'

export const DevResources = (): JSX.Element => {
  const { serverSideProps } = useRouterContext()
  const { host, port } = serverSideProps.devConfig

  let viteBaseUrl = `http://${host}:${port}${VITE_PROXY_PATH}`

  return (
    <>
      <script type="module" async>
        {[
          `import RefreshRuntime from '${viteBaseUrl}/@react-refresh'`,
          'RefreshRuntime.injectIntoGlobalHook(window)',
          'window.$RefreshReg$ = () => {}',
          'window.$RefreshSig$ = () => (type) => type',
          'window.__vite_plugin_react_preamble_installed__ = true',
        ].join('\n')}
      </script>
      <script type="module" async src={`${viteBaseUrl}/@vite/client`}></script>
      <script
        type="module"
        async
        src={`${viteBaseUrl}/client-main.tsx`}
      ></script>
    </>
  )
}
