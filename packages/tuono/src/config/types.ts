import type { AliasOptions, DepOptimizationOptions, Plugin } from 'vite'

export interface TuonoConfig {
  vite?: {
    alias?: AliasOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<Plugin>
  }
}
