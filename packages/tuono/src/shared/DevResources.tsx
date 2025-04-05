import type { JSX } from 'react'

import type { TuonoConfigServer } from '../config'

const VITE_PROXY_PATH = '/vite-server'
const DEFAULT_SERVER_CONFIG = { host: 'localhost', origin: null, port: 3000 }

interface DevResourcesProps {
  devServerConfig?: TuonoConfigServer
}

export const DevResources = ({
  devServerConfig,
}: DevResourcesProps): JSX.Element => {
  const { host, origin, port } = devServerConfig ?? DEFAULT_SERVER_CONFIG

  const viteBaseUrl =
    origin != null
      ? `${origin}${VITE_PROXY_PATH}`
      : `http://${host}:${port}${VITE_PROXY_PATH}`

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
