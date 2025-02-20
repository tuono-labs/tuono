/**
 * Provided by the rust server and used in the ssr env
 * @see tuono-router {@link ServerInitialLocation}
 */
export interface ServerPayloadLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @see crates/tuono_lib/src/payload.rs
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
    origin: string
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
