/**
 * This module is strongly inspired by the remix project.
 *
 * source: https://github.com/remix-run/remix/blob/main/packages/remix-dev/vite/styles.ts
 */
import path from 'path'

import type { ModuleNode, ViteDevServer } from 'vite'

const isCssFile = (file: string): boolean => cssFileRegExp.test(file)

const cssFileRegExp =
  /\.(css|less|sass|scss|styl|stylus|pcss|postcss|sss)(?:$|\?)/

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

const normalizePath = (modulePath: string): string => {
  return modulePath.startsWith('node_modules')
    ? path.join(process.cwd(), modulePath)
    : modulePath
}

export const getStylesForModule = async (
  viteDevServer: ViteDevServer,
  moduleUrl: string,
  /**
   * All the CSS modules are preloaded and saved in this manifest
   */
  cssModulesManifest: Record<string, string>,
): Promise<string | undefined> => {
  const styles: Record<string, string> = {}
  const deps: Set<ModuleNode> = new Set()

  const moduleFilePath = normalizePath(moduleUrl)
  try {
    let node: ModuleNode | undefined =
      await viteDevServer.moduleGraph.getModuleByUrl(moduleFilePath)

    // If the module is only present in the client module graph, the module
    // won't have been found on the first request to the server. If so, we
    // request the module so it's in the module graph, then try again.
    if (!node) {
      try {
        await viteDevServer.transformRequest(moduleFilePath)
      } catch (err) {
        console.error(err)
      }

      node = await viteDevServer.moduleGraph.getModuleByUrl(moduleFilePath)
    }

    if (!node) {
      console.error(`Could not resolve module for file: ${moduleFilePath}`)
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
              // server environment.
              injectQuery(normalizePath(dep.file), 'inline'),
            )
          ).default as string)

        if (css === undefined) {
          throw new Error()
        }

        styles[dep.url] = css
      } catch {
        // this can happen with dynamically imported modules
        console.warn(`Could not load ${dep.file}`)
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

/**
 * This function transform the componentId into a file path.
 * File extension is not required for the vite.moduleGraph URL search.
 */
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
  /**
   * The route name (should match tuono-router specs)
   */
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

  return await getStylesForModule(viteDevServer, fileUrl, cssModulesManifest)
}

/**
 * This function is used to find all the dependencies of a module node.
 * The starting node is always a route.
 */
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
