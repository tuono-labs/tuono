import type { JSX } from 'react'
import { useRouterContext } from 'tuono-router'

const VITE_PROXY_PATH = '/vite-server'
const DEFAULT_SERVER_CONFIG = { host: 'localhost', port: 3000 }

export const DevResources = (): JSX.Element => {
  const { serverPayload } = useRouterContext()
  const { host, port } = serverPayload?.devServerConfig ?? DEFAULT_SERVER_CONFIG

  const viteBaseUrl = `http://${host}:${port}${VITE_PROXY_PATH}`

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
