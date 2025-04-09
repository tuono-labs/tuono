import type {
  AliasOptions,
  DepOptimizationOptions,
  PluginOption,
  CSSOptions,
} from 'vite'

export interface TuonoConfigServer {
  host: string
  origin: string | null
  port: number
}

/**
 * @see http://tuono.dev/documentation/configuration
 */
export interface TuonoConfig {
  server?: Partial<TuonoConfigServer>
  vite?: {
    alias?: AliasOptions
    css?: CSSOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<PluginOption>
  }
}
