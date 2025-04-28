import type { RouteNode } from '../types'

export function removeExt(d: string, keepExtension: boolean = false): string {
  return keepExtension ? d : d.substring(0, d.lastIndexOf('.')) || d
}

export function replaceBackslash(s: string): string {
  return s.replaceAll(/\\/gi, '/')
}

export function cleanPath(pathToClean: string): string {
  // remove double slashes
  return pathToClean.replace(/\/{2,}/g, '/')
}

export function removeUnderscores(s?: string): string | undefined {
  return s?.replaceAll(/(^_|_$)/gi, '').replaceAll(/(\/_|_\/)/gi, '/')
}

export function routePathToVariable(routePath: string): string {
  return (
    removeUnderscores(routePath)
      ?.replace(/\/\$\//g, '/splat/')
      .replace(/\$$/g, 'splat')
      .replace(/\$/g, '')
      .split(/[/-]/g)
      .map((d, i) => (i > 0 ? capitalize(d) : d))
      .join('')
      .replace(/([^a-zA-Z0-9]|[.])/gm, '')
      .replace(/^(\d)/g, 'R$1') ?? ''
  )
}

export function multiSortBy<T>(
  arr: Array<T>,
  accessors: Array<(item: T) => unknown> = [(d): unknown => d],
): Array<T> {
  return arr
    .map((d, i) => [d, i] as const)
    .sort(([a, ai], [b, bi]) => {
      for (const accessor of accessors) {
        const ao = accessor(a)
        const bo = accessor(b)

        if (typeof ao === 'undefined') {
          if (typeof bo === 'undefined') {
            continue
          }
          return 1
        }

        if (ao === bo) {
          continue
        }

        return (ao as number) > (bo as number) ? 1 : -1
      }

      return ai - bi
    })
    .map(([d]) => d)
}

export function capitalize(s: string): string {
  if (typeof s !== 'string') return ''
  return s.charAt(0).toUpperCase() + s.slice(1)
}

export function spaces(d: number): string {
  return Array.from({ length: d })
    .map(() => ' ')
    .join('')
}

export function removeTrailingUnderscores(s?: string): string | undefined {
  return s?.replaceAll(/(_$)/gi, '').replaceAll(/(_\/)/gi, '/')
}

export function removeGroups(s: string): string {
  return s.replaceAll('//', '/')
}

export function trimPathLeft(pathToTrim: string): string {
  return pathToTrim === '/' ? pathToTrim : pathToTrim.replace(/^\/{1,}/, '')
}

export function removeLastSlash(str: string): string {
  if (str.length > 1 && str.endsWith('/')) {
    return str.substring(0, str.length - 1)
  }
  return str
}

/**
 * The `node.path` is used as the `id` in the route definition.
 * This function checks if the given node has a parent and if so, it determines the correct path for the given node.
 * @param node - The node to determine the path for.
 * @returns The correct path for the given node.
 */
export function determineNodePath(node: RouteNode): string {
  return (node.path = node.parent
    ? node.routePath.replace(node.parent.routePath, '') || '/'
    : node.routePath)
}
