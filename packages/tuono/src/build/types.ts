import type { TuonoConfig, TuonoConfigServer } from '../config'

export interface InternalTuonoConfig extends Omit<TuonoConfig, 'server'> {
  server: TuonoConfigServer
}
