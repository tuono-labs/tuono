import type { JSX } from 'react'

const TUONO_DEV_SERVER_PORT = 3000
const VITE_PROXY_PATH = '/vite-server'
const SCRIPT_BASE_URL = `http://localhost:${TUONO_DEV_SERVER_PORT}${VITE_PROXY_PATH}`

export const DevResources = (): JSX.Element => (
  <>
    <script type="module" async>
      {[
        `import RefreshRuntime from '${SCRIPT_BASE_URL}/@react-refresh'`,
        'RefreshRuntime.injectIntoGlobalHook(window)',
        'window.$RefreshReg$ = () => {}',
        'window.$RefreshSig$ = () => (type) => type',
        'window.__vite_plugin_react_preamble_installed__ = true',
      ].join('\n')}
    </script>
    <script
      type="module"
      async
      src={`${SCRIPT_BASE_URL}/@vite/client`}
    ></script>
    <script
      type="module"
      async
      src={`${SCRIPT_BASE_URL}/client-main.tsx`}
    ></script>
  </>
)
