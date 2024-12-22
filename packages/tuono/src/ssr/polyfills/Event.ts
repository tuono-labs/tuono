/**
 * https://dom.spec.whatwg.org/#interface-event
 */
// @ts-expect-error Not all the properties are implemented
export class EventPolyfill implements Event {
  type: string
  bubbles: boolean
  cancelable: boolean
  composed: boolean
  currentTarget: EventTarget | null = null
  defaultPrevented = false
  eventPhase = 0
  isTrusted = false
  target: EventTarget | null = null
  timeStamp: number = Date.now()
  returnValue = true
  srcElement: EventTarget | null = null

  constructor(type: string, eventInitDict?: EventInit) {
    this.type = type
    this.bubbles = eventInitDict?.bubbles ?? false
    this.cancelable = eventInitDict?.cancelable ?? false
    this.composed = eventInitDict?.composed ?? false
  }
}
