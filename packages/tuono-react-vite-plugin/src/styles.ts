/**
 * This module is strongly inspired by the remix project.
 *
 * source: https://github.com/remix-run/remix/blob/main/packages/remix-dev/vite/styles.ts
 */
import path from 'path'

import type { ModuleNode, ViteDevServer } from 'vite'

const isCssFile = (file: string): boolean => cssFileRegExp.test(file)

// Vite doesn't expose these so we just copy the list for now
// https://github.com/vitejs/vite/blob/d6bde8b03d433778aaed62afc2be0630c8131908/packages/vite/src/node/constants.ts#L49C23-L50
const cssFileRegExp =
  /\.(css|less|sass|scss|styl|stylus|pcss|postcss|sss)(?:$|\?)/
// https://github.com/vitejs/vite/blob/d6bde8b03d433778aaed62afc2be0630c8131908/packages/vite/src/node/plugins/css.ts#L160
const cssModulesRegExp = new RegExp(`\\.module${cssFileRegExp.source}`)

const routesFolder = path.relative(process.cwd(), 'src/routes')

const injectQuery = (url: string, query: string): string =>
  url.includes('?') ? url.replace('?', `?${query}&`) : `${url}?${query}`

export const isCssModulesFile = (file: string): boolean =>
  cssModulesRegExp.test(file)

const cssUrlParamsWithoutSideEffects = ['url', 'inline', 'raw', 'inline-css']

const isCssUrlWithoutSideEffects = (url: string): boolean => {
  const queryString = url.split('?')[1]

  if (!queryString) {
    return false
  }

  const params = new URLSearchParams(queryString)
  for (const paramWithoutSideEffects of cssUrlParamsWithoutSideEffects) {
    if (
      // Parameter is blank and not explicitly set, i.e. "?url", not "?url="
      params.get(paramWithoutSideEffects) === '' &&
      !url.includes(`?${paramWithoutSideEffects}=`) &&
      !url.includes(`&${paramWithoutSideEffects}=`)
    ) {
      return true
    }
  }

  return false
}

function findFileFromComponentId(id: string): string {
  if (id.endsWith('/')) {
    return id + 'index'
  }

  if (id.includes('__root__')) {
    return id.replaceAll('__root__', '__layout')
  }

  return id
}

export const getStylesForComponentId = async (
  viteDevServer: ViteDevServer,
  componentId: string | null,
  /**
   * All the CSS modules are preloaded and saved in this manifest
   */
  cssModulesManifest: Record<string, string>,
): Promise<string | undefined> => {
  const relativeFilePath = path.join(
    routesFolder,
    findFileFromComponentId(componentId || ''),
  )

  const fileUrl = path.join(process.cwd(), relativeFilePath)

  const styles: Record<string, string> = {}
  const deps: Set<ModuleNode> = new Set()

  try {
    let node: ModuleNode | undefined =
      await viteDevServer.moduleGraph.getModuleByUrl(fileUrl)

    // If the module is only present in the client module graph, the module
    // won't have been found on the first request to the server. If so, we
    // request the module so it's in the module graph, then try again.
    if (!node) {
      try {
        await viteDevServer.transformRequest(fileUrl)
      } catch (err) {
        console.error(err)
      }

      node = await viteDevServer.moduleGraph.getModuleByUrl(fileUrl)
    }

    if (!node) {
      console.error(`Could not resolve module for file: ${fileUrl}`)
      return
    }
    await findNodeDependencies(viteDevServer, node, deps)
  } catch (error) {
    console.error(error)
  }

  for (const dep of deps) {
    if (
      dep.file &&
      isCssFile(dep.file) &&
      !isCssUrlWithoutSideEffects(dep.url) // Ignore styles that resolved as URLs, inline or raw. These shouldn't get injected.
    ) {
      try {
        const css = isCssModulesFile(dep.file)
          ? cssModulesManifest[dep.file]
          : ((
              await viteDevServer.ssrLoadModule(
                // We need the ?inline query in Vite v6 when loading CSS in SSR
                // since it does not expose the default export for CSS in a
                // server environment. This is to align with non-SSR
                // environments. For backwards compatibility with v5 we keep
                // using the URL without ?inline query because the HMR code was
                // relying on the implicit SSR-client module graph relationship.
                injectQuery(dep.url, 'inline'),
              )
            ).default as string)

        if (css === undefined) {
          throw new Error()
        }

        styles[dep.url] = css
      } catch {
        console.warn(`Could not load ${dep.file}`)
        // this can happen with dynamically imported modules, I think
        // because the Vite module graph doesn't distinguish between
        // static and dynamic imports? TODO investigate, submit fix
      }
    }
  }

  return (
    Object.entries(styles)
      .map(([fileName, css]) => [
        `\n/* ${fileName
          // Escape comment syntax in file paths
          .replace(/\/\*/g, '/\\*')
          .replace(/\*\//g, '*\\/')} */`,
        css,
      ])
      .flat()
      .join('\n') || undefined
  )
}

const findNodeDependencies = async (
  vite: ViteDevServer,
  node: ModuleNode,
  deps: Set<ModuleNode>,
): Promise<void> => {
  // since `ssrTransformResult.deps` contains URLs instead of `ModuleNode`s, this process is asynchronous.
  // instead of using `await`, we resolve all branches in parallel.
  const branches: Array<Promise<void>> = []

  async function addFromNode(innerNode: ModuleNode): Promise<void> {
    if (!deps.has(innerNode)) {
      deps.add(innerNode)
      await findNodeDependencies(vite, innerNode, deps)
    }
  }

  async function addFromUrl(url: string): Promise<void> {
    const innerNode = await vite.moduleGraph.getModuleByUrl(url)

    if (innerNode) {
      await addFromNode(innerNode)
    }
  }

  if (node.ssrTransformResult) {
    if (node.ssrTransformResult.deps) {
      node.ssrTransformResult.deps.forEach((url) =>
        branches.push(addFromUrl(url)),
      )
    }
  } else {
    node.importedModules.forEach((innerNode: ModuleNode) =>
      branches.push(addFromNode(innerNode)),
    )
  }

  await Promise.all(branches)
}
