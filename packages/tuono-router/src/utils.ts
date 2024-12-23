import type { Segment } from './types'

export function joinPaths(paths: Array<string | undefined>): string {
  return cleanPath(paths.filter(Boolean).join('/'))
}

function cleanPath(path: string): string {
  // remove double slashes
  return path.replace(/\/{2,}/g, '/')
}

export function parsePathname(pathname?: string): Array<Segment> {
  if (!pathname) {
    return []
  }

  pathname = cleanPath(pathname)

  const segments: Array<Segment> = []

  if (pathname.slice(0, 1) === '/') {
    pathname = pathname.substring(1)
    segments.push({
      type: 'pathname',
      value: '/',
    })
  }

  if (!pathname) {
    return segments
  }

  // Remove empty segments and '.' segments
  const split = pathname.split('/').filter(Boolean)

  segments.push(
    ...split.map((part): Segment => {
      if (part === '$' || part === '*') {
        return {
          type: 'wildcard',
          value: part,
        }
      }

      if (part.charAt(0) === '$') {
        return {
          type: 'param',
          value: part,
        }
      }

      return {
        type: 'pathname',
        value: part,
      }
    }),
  )

  if (pathname.slice(-1) === '/') {
    pathname = pathname.substring(1)
    segments.push({
      type: 'pathname',
      value: '/',
    })
  }

  return segments
}
export function trimPathLeft(path: string): string {
  return path === '/' ? path : path.replace(/^\/{1,}/, '')
}
export function trimPathRight(path: string): string {
  return path === '/' ? path : path.replace(/\/{1,}$/, '')
}
export function trimPath(path: string): string {
  return trimPathRight(trimPathLeft(path))
}
