import { describe, it, expect, vi } from 'vitest'

import { MessageChannelPolyfill, MessagePortPolyfill } from './MessageChannel'

describe('MessagePortPolyfill', () => {
  it('should invoke onmessage when a message is posted', () => {
    const port = new MessagePortPolyfill()

    const onmessageMock = vi.fn<EventListener>()

    port.onmessage = onmessageMock

    port.dispatchEvent({ data: 'Hello, world!' } as MessageEvent)

    expect(onmessageMock).toHaveBeenCalledOnce()
    expect(onmessageMock).toHaveBeenCalledWith({ data: 'Hello, world!' })
  })

  it('should handle multiple event listeners', () => {
    const port = new MessagePortPolyfill()

    const listener1 = vi.fn<EventListener>()
    const listener2 = vi.fn<EventListener>()

    port.addEventListener('message', listener1)
    port.addEventListener('message', listener2)

    port.dispatchEvent({ data: 'Test message' } as MessageEvent)

    expect(listener1).toHaveBeenCalledOnce()
    expect(listener1).toHaveBeenCalledWith({ data: 'Test message' })

    expect(listener2).toHaveBeenCalledOnce()
    expect(listener2).toHaveBeenCalledWith({ data: 'Test message' })
  })

  it('should not invoke removed event listeners', () => {
    const port = new MessagePortPolyfill()

    const listener = vi.fn<EventListener>()

    port.addEventListener('message', listener)
    port.dispatchEvent({ data: 'First message' } as MessageEvent)

    port.removeEventListener('message', listener)
    port.dispatchEvent({ data: 'Second message' } as MessageEvent)

    expect(listener).toHaveBeenCalledOnce()
    expect(listener).toHaveBeenCalledWith({ data: 'First message' })
  })

  it('should not post messages if otherPort is null', () => {
    const port = new MessagePortPolyfill()

    const listener = vi.fn<EventListener>()

    port.onmessage = listener

    port.postMessage('Hello!')

    expect(listener).not.toHaveBeenCalledOnce()
  })
})

describe('MessageChannelPolyfill', () => {
  it('should send and receive messages between ports', () => {
    const channel = new MessageChannelPolyfill()

    const listener = vi.fn<EventListener>()

    channel.port1.onmessage = listener

    channel.port2.postMessage('Hello, port1!')
    channel.port2.postMessage('How are you?')

    expect(listener).toHaveBeenCalledTimes(2)
    expect(listener).toHaveBeenNthCalledWith(
      1,
      expect.objectContaining({ data: 'Hello, port1!' }),
    )
    expect(listener).toHaveBeenNthCalledWith(
      2,
      expect.objectContaining({ data: 'How are you?' }),
    )
  })

  it('should support addEventListener and removeEventListener', () => {
    const channel = new MessageChannelPolyfill()

    const listener = vi.fn<EventListener>()

    channel.port1.addEventListener('message', listener)
    channel.port2.postMessage('Hello, port1!')

    expect(listener).toHaveBeenNthCalledWith(
      1,
      expect.objectContaining({ data: 'Hello, port1!' }),
    )

    channel.port1.removeEventListener('message', listener)
    channel.port2.postMessage('Message after removing listener')

    expect(listener).not.toHaveBeenCalledTimes(2)
  })

  it('should handle bidirectional communication between ports', () => {
    const channel = new MessageChannelPolyfill()

    const listener1 = vi.fn<EventListener>()
    const listener2 = vi.fn<EventListener>()

    channel.port1.onmessage = listener1
    channel.port2.onmessage = listener2

    channel.port1.postMessage('Hello, port2!')
    channel.port2.postMessage('Hello, port1!')

    expect(listener1).toHaveBeenCalledOnce()
    expect(listener1).toHaveBeenCalledWith(
      expect.objectContaining({ data: 'Hello, port1!' }),
    )

    expect(listener2).toHaveBeenCalledOnce()
    expect(listener2).toHaveBeenCalledWith(
      expect.objectContaining({ data: 'Hello, port2!' }),
    )
  })
})

describe('MessagePort', () => {
  it('should not send a message on close', () => {
    const { port1, port2 } = new MessageChannelPolyfill()

    const listener = vi.fn<EventListener>()

    port1.onmessage = listener

    port2.postMessage('Test message')

    expect(listener).toHaveBeenCalledOnce()
    expect(listener).toHaveBeenCalledWith(
      expect.objectContaining({ data: 'Test message' }),
    )

    port1.close()
    port2.postMessage('Another message')

    expect(listener).not.toHaveBeenCalledTimes(2)
  })
})
