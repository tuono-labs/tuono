import type { AliasOptions, DepOptimizationOptions, PluginOption } from 'vite'

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
