/**
 * POLYFILLS START HERE ---------------------------------------
 *
 * Tuono internally uses a V8 JS engine that implements very few
 * browser/node/deno APIs in order to make it super fast and
 * share it within a multi thread runtime.
 *
 * While this is the reason of its speed some JS APIs needed for server side rendering are
 * still required to be polyfilled.
 *
 * We basically have three ways to polyfill APIs:
 * 1. Create them with rust and expose them directly through the V8 engine to
 * the JS source.
 * 2. Polyfill them at the beginning of the JS source
 * 3. Inject them when needed with rollup-inject plugin
 *
 * Why not all the libraries can be just injected with rollup-inject?
 *
 * Unfortunately the following APIs are JS classes so leaving to rollup the
 * duty of linking them can cause to declare them after their usage.
 *
 * Classes are not hoisted leading then to a ReferenceError.
 *
 * The best solution is to create these polyfills within the rust environment
 * and share the classes in the JS scope by passing them through the V8 engine (best for speed and
 * code quality).
 *
 * This function might be a good entry point for adding such polyfills
 * https://docs.rs/ssr_rs/latest/ssr_rs/struct.Ssr.html#method.add_global_fn
 */
import 'fast-text-encoding'
import { MessageChannelPolyfill } from './polyfills/MessageChannel'

// eslint-disable-next-line import/newline-after-import
;(function (
  scope: Partial<Pick<typeof globalThis, 'MessageChannel'>> = {},
): void {
  scope['MessageChannel'] = scope['MessageChannel'] ?? MessageChannelPolyfill
})(this)

/**
 * POLYFILLS END HERE ----------------------------------------
 */
import type { ReadableStream } from 'node:stream/web'

import type { ReactNode } from 'react'
import { renderToReadableStream } from 'react-dom/server'
import { RouterProvider, createRouter } from 'tuono-router'
import type { createRoute } from 'tuono-router'

import { streamToString } from './utils'

type RouteTree = ReturnType<typeof createRoute>
type Mode = 'Dev' | 'Prod'

const TUONO_DEV_SERVER_PORT = 3000
const VITE_PROXY_PATH = '/vite-server'
const SCRIPT_BASE_URL = `http://localhost:${TUONO_DEV_SERVER_PORT}${VITE_PROXY_PATH}`

const ViteScripts = (): ReactNode => (
  <>
    <script type="module">
      {[
        `import RefreshRuntime from '${SCRIPT_BASE_URL}/@react-refresh'`,
        'RefreshRuntime.injectIntoGlobalHook(window)',
        'window.$RefreshReg$ = () => {}',
        'window.$RefreshSig$ = () => (type) => type',
        'window.__vite_plugin_react_preamble_installed__ = true',
      ].join('\n')}
    </script>
    <script type="module" src={`${SCRIPT_BASE_URL}/@vite/client`}></script>
    <script type="module" src={`${SCRIPT_BASE_URL}/client-main.tsx`}></script>
  </>
)

interface ProductionBunldesProps {
  bundles: Array<string>
  mode: Mode
}

const ProductionCssLinks = ({
  bundles,
  mode,
}: ProductionBunldesProps): ReactNode => {
  if (mode === 'Dev') return null
  return (
    <>
      {bundles.map((cssHref) => (
        <link rel="stylesheet" type="text/css" href={`/${cssHref}`} />
      ))}
    </>
  )
}

const ProductionScriptLinks = ({
  bundles,
  mode,
}: ProductionBunldesProps): ReactNode => {
  if (mode === 'Dev') return null
  return (
    <>
      {bundles.map((scriptSrc) => (
        <script type="module" src={`/${scriptSrc}`}></script>
      ))}
    </>
  )
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
        <ProductionCssLinks mode={mode} bundles={cssBundles} />
        <ProductionScriptLinks mode={mode} bundles={jsBundles} />
        <RouterProvider router={router} serverProps={serverProps as never} />
        {mode === 'Dev' && <ViteScripts />}
        <script>{`window.__TUONO_SSR_PROPS__=${payload as string}`}</script>
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
