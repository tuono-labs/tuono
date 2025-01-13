/**
 * This component is heavily inspired by Next.js dynamic function
 * Link: https://github.com/vercel/next.js/blob/1df81bcea62800198884438a2bb27ba14c9d506a/packages/next/src/shared/lib/dynamic.tsx
 */
import * as React from 'react'

const isServerSide = typeof window === 'undefined'

type ComponentModule<P = {}> = { default: React.ComponentType<P> }

interface DynamicOptions {
  ssr?: boolean
  loading?: React.ComponentType<any> | null
}

type Loader = () => Promise<React.ComponentType<any> | ComponentModule<any>>

interface LoadableOptions extends DynamicOptions {
  loader: Loader
}

export type LoadableFn<P = {}> = (
  opts: LoadableOptions,
) => React.ComponentType<P>

const defaultLoaderOptions: LoadableOptions = {
  ssr: true,
  loading: null,
  loader: () => Promise.resolve(() => null),
}

function noSSR<P = {}>(
  LoadableInitializer: LoadableFn<P>,
  loadableOptions: LoadableOptions,
): React.ComponentType<P> {
  if (!isServerSide) {
    return LoadableInitializer(loadableOptions)
  }

  if (!loadableOptions.loading) return () => null

  const Loading = loadableOptions.loading
  // This will only be rendered on the server side
  return () => <Loading />
}

const Loadable = (options: LoadableOptions) => {
  const opts = { ...defaultLoaderOptions, ...options }
  const Lazy = React.lazy(() => opts.loader().then())
  const Loading = opts.loading

  function LoadableComponent(props: any): React.JSX.Element {
    const fallbackElement = Loading ? <Loading /> : null

    const Wrap = Loading ? React.Suspense : React.Fragment
    const wrapProps = Loading ? { fallback: fallbackElement } : {}

    // TODO: In case ssr = false handle also the assets preloading
    return (
      <Wrap {...wrapProps}>
        <Lazy {...props} />
      </Wrap>
    )
  }
  LoadableComponent.displayName = 'LoadableComponent'

  return LoadableComponent
}

/**
 * This function lets you dynamically import a component.
 * It uses [React.lazy()](https://react.dev/reference/react/lazy) with [Suspense](https://react.dev/reference/react/Suspense) under the hood.
 */
export const dynamic = <P = {},>(
  importFn: Loader,
  opts?: DynamicOptions,
): React.ComponentType<P> => {
  if (typeof opts?.ssr === 'boolean' && !opts?.ssr) {
    return noSSR(Loadable, { ...opts, loader: importFn })
  }
  return Loadable({ ...opts, loader: importFn })
}
