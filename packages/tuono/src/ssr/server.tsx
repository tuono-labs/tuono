import 'fast-text-encoding' // Mandatory for React18
import { MessageChannelPolyfill } from './polyfills/MessageChannel'
  ; (function(scope = {}) {
    scope['MessageChannel'] = MessageChannelPolyfill
  })(
    typeof window !== 'undefined'
      ? window
      : typeof global !== 'undefined'
        ? global
        : this,
  )
import type { ReadableStream } from 'node:stream/web'

import { renderToReadableStream } from 'react-dom/server'
import { RouterProvider, createRouter } from 'tuono-router'
import type { createRoute } from 'tuono-router'

import { streamToString } from './utils'
import { ReactNode, Suspense } from 'react'

type RouteTree = ReturnType<typeof createRoute>
type Mode = 'Dev' | 'Prod'

const TUONO_DEV_SERVER_PORT = 3000
const VITE_PROXY_PATH = '/vite-server'

const VITE_DEV_AND_HMR = `import RefreshRuntime from 'http://localhost:${TUONO_DEV_SERVER_PORT}${VITE_PROXY_PATH}/@react-refresh'
RefreshRuntime.injectIntoGlobalHook(window)
window.$RefreshReg$ = () => {}
window.$RefreshSig$ = () => (type) => type
window.__vite_plugin_react_preamble_installed__ = true`

const ViteScripts = (): ReactNode => (
  <>
    <script type="module">{VITE_DEV_AND_HMR}</script>
    <script
      type="module"
      src={`http://localhost:${TUONO_DEV_SERVER_PORT}${VITE_PROXY_PATH}/@vite/client`}
    ></script>
    <script
      type="module"
      src={`http://localhost:${TUONO_DEV_SERVER_PORT}${VITE_PROXY_PATH}/client-main.tsx`}
    ></script>
  </>
)

function generateCssLinks(cssBundles: Array<string>, mode: Mode): string {
  if (mode === 'Dev') return ''
  return cssBundles.reduce((acc, value) => {
    return acc + `<link rel="stylesheet" type="text/css" href="/${value}" />`
  }, '')
}

function generateJsScripts(jsBundles: Array<string>, mode: Mode): string {
  if (mode === 'Dev') return ''
  return jsBundles.reduce((acc, value) => {
    return acc + `<script type="module" src="/${value}"></script>`
  }, '')
}

export function serverSideRendering(routeTree: RouteTree) {
  return async function render(payload: string | undefined): Promise<string> {
    const serverProps = (payload ? JSON.parse(payload) : {}) as Record<
      string,
      unknown
    >

    const mode = serverProps.mode as Mode
    const jsBundles = serverProps.jsBundles as Array<string>
    const cssBundles = serverProps.cssBundles as Array<string>
    const router = createRouter({ routeTree }) // Render the app

    const stream = await renderToReadableStream(
      <>
        <RouterProvider router={router} serverProps={serverProps as never} />
        {mode === 'Dev' && <ViteScripts />}
        <script
          dangerouslySetInnerHTML={{
            __html: `window.__TUONO_SSR_PROPS__=${payload as string}`,
          }}
        />
      </>,
    )

    await stream.allReady

    return await streamToString(
      // ReadableStream should be implemented in node)
      stream as unknown as ReadableStream<Uint8Array>,
    )

    //return `<!doctype html>
    //<html ${helmet.htmlAttributes.toString()}>
    //<head>
    //${helmet.title.toString()}
    //${helmet.priority.toString()}
    //${helmet.meta.toString()}
    //${helmet.link.toString()}
    //${helmet.script.toString()}
    //${generateCssLinks(cssBundles, mode)}
    //</head>
    //<body ${helmet.bodyAttributes.toString()}>
    //<div id="__tuono">${app}</div>
    //${renderToStaticMarkup(
    //<script
    //dangerouslySetInnerHTML={{
    //__html: `window.__TUONO_SSR_PROPS__=${payload as string}`,
    //}}
    ///>,
    //)}
    //${generateJsScripts(jsBundles, mode)}
    //${mode === 'Dev' ? VITE_DEV_AND_HMR : ''}
    //</body>
    //</html>
    //`
  }
}
