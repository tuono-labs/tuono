// #region POLYFILLS
/**
 * Tuono internally uses a V8 JS engine that implements very few
 * browser/node/deno APIs in order to make it super fast and
 * share it within a multi thread runtime.
 *
 * While this is the reason of its speed, server side rendering
 * requires some JS APIs that need to be polyfilled.
 *
 * We basically have three ways to polyfill APIs:
 * 1. Create them with rust and expose them directly through the V8 engine to
 *    the JS source.
 * 2. Polyfill them at the beginning of the JS source
 *    (what we are doing here)
 * 3. Inject them via rollup-inject plugin, when needed
 *
 * Q: Why not all the libraries can be just injected with rollup-inject?
 * A: Leaving to rollup the duty of linking them can cause to declare them after their usage.
 *    The following APIs are JS classes, and are not hoisted, hence this might
 *    cause ReferenceError(s).
 *
 * The best solution is to create these polyfills within the rust environment
 * and share the classes in the JS scope by passing them through the V8 engine
 * (best for speed and code quality).
 *
 * This function might be a good entry point for adding such polyfills
 * https://docs.rs/ssr_rs/latest/ssr_rs/struct.Ssr.html#method.add_global_fn
 */
import 'fast-text-encoding'

/* eslint-disable import/order, import/newline-after-import */
import { MessageChannelPolyfill } from './polyfills/MessageChannel'
;(function (
  scope: Partial<Pick<typeof globalThis, 'MessageChannel'>> = {},
): void {
  scope['MessageChannel'] = scope['MessageChannel'] ?? MessageChannelPolyfill
})(this)
/* eslint-enable import/order, import/newline-after-import */
// #endregion POLYFILLS

import type { ReadableStream } from 'node:stream/web'

import { renderToReadableStream } from 'react-dom/server'
import { RouterProvider, createRouter } from 'tuono-router'
import type { createRoute } from 'tuono-router'

import { DevResources } from './components/DevResources'
import { ProdResources } from './components/ProdResources'
import type { Mode } from './types'
import { streamToString } from './utils'

type RouteTree = ReturnType<typeof createRoute>

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

        {mode === 'Dev' && <DevResources />}
        {mode === 'Prod' && (
          <ProdResources cssBundles={cssBundles} jsBundles={jsBundles} />
        )}

        <script>{`window.__TUONO_SSR_PROPS__=${payload as string}`}</script>
      </>,
    )

    await stream.allReady

    return await streamToString(
      // ReadableStream should be implemented in node)
      stream as unknown as ReadableStream<Uint8Array>,
    )
  }
}
