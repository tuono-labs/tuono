import type { AliasOptions, DepOptimizationOptions, PluginOption } from 'vite'

export interface TuonoConfig {
  vite?: {
    alias?: AliasOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<PluginOption>
  }
}
