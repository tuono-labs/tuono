import type {
  AliasOptions,
  DepOptimizationOptions,
  PluginOption,
  CSSOptions,
} from 'vite'

/**
 * @see http://tuono.dev/documentation/configuration
 */
export interface TuonoConfig {
  server?: {
    host?: string
    origin?: string
    port?: number
  }
  vite?: {
    alias?: AliasOptions
    css?: CSSOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<PluginOption>
  }
}
