import { EventPolyfill } from './Event'

/**
 * https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface
 */
// @ts-expect-error Not all the properties are implemented
export class MessageEventPolyfill<T>
  extends EventPolyfill
  implements MessageEvent
{
  data?: T
  lastEventId: string
  origin: string
  ports: ReadonlyArray<MessagePort>
  source: MessageEventSource | null

  constructor(type: string, options: MessageEventInit<T>) {
    super(type, options)
    this.data = options.data
    this.lastEventId = options.lastEventId ?? ''
    this.origin = options.origin ?? ''
    this.ports = options.ports ?? []
    this.source = options.source ?? null
  }
}
