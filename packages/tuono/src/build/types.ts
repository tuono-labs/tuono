import type { TuonoConfig } from '../config'

export interface InternalTuonoConfig extends Omit<TuonoConfig, 'server'> {
  server: Required<NonNullable<TuonoConfig['server']>>
}
