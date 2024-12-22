/* Modified from https://github.com/rocwind/message-port-polyfill/blob/master/src/index.ts
 * MIT License
 *
 * Copyright (c) 2019 Roc
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

export class MessagePortPolyfill implements MessagePort {
  onmessage: ((this: MessagePort, ev: MessageEvent) => unknown) | null = null
  /** @warning this is declared to satisfy {@link MessagePort} interface requirements but is never called  */
  onmessageerror: ((this: MessagePort, ev: MessageEvent) => unknown) | null =
    null

  otherPort: MessagePortPolyfill | null = null

  private onmessageListeners: Array<(ev: MessageEvent) => void> = []
  private isClosed = false

  dispatchEvent(event: MessageEvent): boolean {
    if (this.isClosed) return false
    if (this.onmessage) {
      this.onmessage(event)
    }
    this.onmessageListeners.forEach((listener) => {
      listener(event)
    })
    return true
  }

  postMessage(message: unknown): void {
    if (this.isClosed || !this.otherPort) return

    const event = new MessageEvent('message', { data: message })
    this.otherPort.dispatchEvent(event)
  }

  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
  ): void {
    if (this.isClosed || type !== 'message') return

    if (
      typeof listener === 'function' &&
      !this.onmessageListeners.includes(listener)
    ) {
      this.onmessageListeners.push(listener)
    }
  }

  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
  ): void {
    if (this.isClosed || type !== 'message') return

    if (typeof listener === 'function') {
      const index = this.onmessageListeners.indexOf(listener)
      if (index !== -1) {
        this.onmessageListeners.splice(index, 1)
      }
    }
  }

  start(): void {
    // do nothing at this moment
  }

  close(): void {
    this.isClosed = true
  }
}

export class MessageChannelPolyfill implements MessageChannel {
  readonly port1: MessagePortPolyfill
  readonly port2: MessagePortPolyfill

  constructor() {
    this.port1 = new MessagePortPolyfill()
    this.port2 = new MessagePortPolyfill()

    this.port1.otherPort = this.port2
    this.port2.otherPort = this.port1
  }
}
