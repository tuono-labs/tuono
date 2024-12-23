import type { ParsedLocation } from '../components/RouterContext'

export function fromUrlToParsedLocation(href: string): ParsedLocation {
  const location = new URL(href, window.location.origin)
  return {
    href: location.href,
    pathname: location.pathname,
    search: Object.fromEntries(location.searchParams),
    searchStr: location.search,
    hash: location.hash,
  }
}
