export interface ServerPayloadLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @see crates/tuono_lib/src/payload.rs
 * @warning keep in sync with the same interface inside tuono-router until router is not specialized
 */
export interface ServerPayload<TData = unknown> {
  mode: 'Prod' | 'Dev'

  location: ServerPayloadLocation

  data: TData

  /** Available only on 'Prod' mode */
  jsBundles: Array<string> | null
  cssBundles: Array<string> | null

  /** Available only on 'Dev' mode */
  devServerConfig?: {
    port: number
    host: string
  }
}

/* the above type could be refined with an union like this
(
  | {
      mode: 'Prod'
      jsBundles: Array<string>
      cssBundles: Array<string>
    }
  | {
      mode: 'Dev'
      devServerConfig: {
        port: number
        host: string
      }
    }
)
*/

export interface TuonoProps<T> {
  data?: T
  isLoading: boolean
}
