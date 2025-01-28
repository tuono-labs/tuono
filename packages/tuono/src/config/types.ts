import type { AliasOptions, DepOptimizationOptions, PluginOption } from 'vite'

/**
 * @see http://tuono.dev/documentation/configuration
 */
export interface TuonoConfig {
  server?: {
    host?: string
    port?: number
  }
  vite?: {
    alias?: AliasOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<PluginOption>
  }
}
