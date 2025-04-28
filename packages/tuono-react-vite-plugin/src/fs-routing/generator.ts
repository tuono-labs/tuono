import * as fsp from 'fs/promises'
import path from 'path'

import { format } from 'prettier'

import type { Config, RouteNode } from '../types'

import { buildRouteConfig } from './build-route-config'
import { hasParentRoute } from './has-parent-route'
import {
  cleanPath,
  determineNodePath,
  multiSortBy,
  replaceBackslash,
  removeExt,
  routePathToVariable,
  removeGroups,
  removeLastSlash,
  removeUnderscores,
} from './utils'

import {
  ROUTES_FOLDER,
  LAYOUT_PATH_ID,
  GENERATED_ROUTE_TREE,
  DYNAMIC_FN,
} from './constants'

import { sortRouteNodes } from './sort-route-nodes'
import isDefaultExported from './is-default-exported'

let latestTask = 0

const defaultConfig: Config = {
  folderName: ROUTES_FOLDER,
  generatedRouteTree: GENERATED_ROUTE_TREE,
}

let isFirst = false
let skipMessage = false

async function getRouteNodes(
  config = defaultConfig,
): Promise<{ routeNodes: Array<RouteNode>; rustHandlersNodes: Array<string> }> {
  const routeNodes: Array<RouteNode> = []
  const rustHandlersNodes: Array<string> = []

  async function recurse(dir: string): Promise<void> {
    const fullDir = path.resolve(config.folderName, dir)
    const dirList = await fsp.readdir(fullDir, { withFileTypes: true })

    await Promise.all(
      dirList.map(async (dirent) => {
        const fullPath = path.join(fullDir, dirent.name)
        const relativePath = path.join(dir, dirent.name)

        if (dirent.isDirectory()) {
          await recurse(relativePath)
        } else if (fullPath.match(/\.(tsx|ts|jsx|js|mdx)$/)) {
          // Check that the route is correctly default exported
          if (
            fullPath.match(/\.(tsx|ts|jsx|js)$/) &&
            !isDefaultExported((await fsp.readFile(fullPath)).toString())
          ) {
            return
          }
          const filePath = replaceBackslash(path.join(dir, dirent.name))
          const filePathNoExt = removeExt(filePath)
          let routePath = cleanPath(`/${filePathNoExt}`) || ''

          const variableName = routePathToVariable(routePath)

          // Remove the index from the route path and
          // if the route path is empty, use `/'
          if (routePath === 'index') {
            routePath = '/'
          }

          routePath = routePath.replace(/\/index$/, '/') || '/'

          routeNodes.push({
            filePath,
            fullPath,
            routePath,
            variableName,
          })
        } else if (fullPath.match(/\.(rs)$/)) {
          const filePath = replaceBackslash(path.join(dir, dirent.name))
          const filePathNoExt = removeExt(filePath)
          let routePath = cleanPath(`/${filePathNoExt}`) || ''

          if (routePath === 'index') {
            routePath = '/'
          }

          routePath = routePath.replace(/\/index$/, '/') || '/'

          rustHandlersNodes.push(routePath)
        }
      }),
    )
  }

  await recurse('./')

  return { routeNodes, rustHandlersNodes }
}

export async function routeGenerator(
  config: Config = defaultConfig,
): Promise<void> {
  if (!isFirst) {
    isFirst = true
  } else if (skipMessage) {
    skipMessage = false
  }

  const checkLatest = (): boolean => {
    if (latestTask !== taskId) {
      skipMessage = true
      return false
    }

    return true
  }

  const taskId = latestTask + 1
  latestTask = taskId

  const { routeNodes: beforeRouteNodes, rustHandlersNodes } =
    await getRouteNodes(config)

  const preRouteNodes = sortRouteNodes(beforeRouteNodes)

  const routeNodes: Array<RouteNode> = []

  // Loop over the flat list of routeNodes and
  // build up a tree based on the routeNodes' routePath
  const handleNode = (node: RouteNode): void => {
    const parentRoute = hasParentRoute(routeNodes, node, node.routePath)

    if (parentRoute) node.parent = parentRoute

    node.path = determineNodePath(node)

    node.cleanedPath = removeLastSlash(
      removeGroups(removeUnderscores(node.path) ?? ''),
    )

    if (node.parent) {
      node.parent.children = node.parent.children ?? []
      node.parent.children.push(node)
    }
    routeNodes.push(node)
  }

  for (const node of preRouteNodes) {
    handleNode(node)
  }

  const routeConfigChildrenText = buildRouteConfig(routeNodes)

  const sortedRouteNodes = multiSortBy(routeNodes, [
    (d): number => (d.routePath.includes(`/${LAYOUT_PATH_ID}`) ? -1 : 1),
    (d): number => d.routePath.split('/').length,
    (d): number => (d.routePath.endsWith("index'") ? -1 : 1),
    (d): RouteNode => d,
  ])

  const imports = [
    ...sortedRouteNodes.map((node) => {
      const extension = node.filePath.endsWith('mdx') ? '.mdx' : ''
      return `const ${
        node.variableName as string
      }Import = ${DYNAMIC_FN}(() => import('./${replaceBackslash(
        removeExt(
          path.relative(
            path.dirname(config.generatedRouteTree),
            path.resolve(config.folderName, node.filePath),
          ),
          false,
        ),
      )}${extension}'))`
    }),
  ].join('\n')

  const createRoutes = [
    ...sortedRouteNodes.map((node) => {
      const isRoot = node.routePath.endsWith(LAYOUT_PATH_ID)
      const rootDeclaration = isRoot ? ', isRoot: true' : ''
      const variableName = node.variableName as string

      return `const ${variableName} = createRoute({ component: ${variableName}Import${rootDeclaration} })`
    }),
  ].join('\n')

  const createRouteUpdates = [
    sortedRouteNodes
      .map((node) => {
        const variableName = node.variableName as string
        const cleanedPath = node.cleanedPath as string
        return [
          `const ${variableName}Route = ${variableName}.update({
          ${[
            !node.path?.endsWith(LAYOUT_PATH_ID) && `path: '${cleanedPath}'`,
            `getParentRoute: () => ${node.parent?.variableName ?? 'root'}Route`,
            rustHandlersNodes.includes(node.path || '')
              ? 'hasHandler: true'
              : '',
          ]
            .filter(Boolean)
            .join(',')}
        })`,
        ].join('')
      })
      .join('\n\n'),
  ]

  const routeImports = [
    '// This file is auto-generated by Tuono',
    `import { createRoute, ${DYNAMIC_FN} } from 'tuono'`,
    [
      `import RootLayoutImport from './${replaceBackslash(
        path.relative(
          path.dirname(config.generatedRouteTree),
          path.resolve(config.folderName, LAYOUT_PATH_ID),
        ),
      )}'`,
    ].join('\n'),
    imports,
    'const rootRoute = createRoute({ isRoot: true, component: RootLayoutImport });',
    createRoutes,
    '// Create/Update Routes',
    createRouteUpdates,
    '// Create and export the route tree',
    `export const routeTree = rootRoute.addChildren([${routeConfigChildrenText}])`,
  ]
    .filter(Boolean)
    .join('\n\n')

  const routeConfigFileContent = await format(routeImports, {
    semi: false,
    singleQuote: true,
    parser: 'typescript',
  })

  const routeTreeContent = await fsp
    .readFile(path.resolve(config.generatedRouteTree), 'utf-8')
    .catch((e: unknown) => {
      const err = e as Error & { code?: string }
      if (err.code === 'ENOENT') {
        return undefined
      }
      throw err
    })

  if (!checkLatest()) return

  if (routeTreeContent !== routeConfigFileContent) {
    await fsp.mkdir(path.dirname(path.resolve(config.generatedRouteTree)), {
      recursive: true,
    })
    if (!checkLatest()) return
    await fsp.writeFile(
      path.resolve(config.generatedRouteTree),
      routeConfigFileContent,
    )
  }
}
