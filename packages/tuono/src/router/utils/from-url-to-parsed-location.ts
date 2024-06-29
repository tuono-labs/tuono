import type { ParsedLocation } from '../hooks/useRouterStore'

export function fromUrlToParsedLocation(href: string): ParsedLocation {
  /*
   * This function works on both server and client.
   * For this reason we can't rely on the browser's URL api
   */
  return {
    href,
    pathname: href,
    search: undefined,
    searchStr: '',
    hash: '',
  }
}
