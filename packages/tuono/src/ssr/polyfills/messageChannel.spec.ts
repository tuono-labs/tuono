import { describe, it, expect } from 'vitest'

import { MessageChannelPolyfill, MessagePortPolyfill } from './messageChannel'

describe('MessagePortPolyfill', () => {
  it('should invoke onmessage when a message is posted', () => {
    const port = new MessagePortPolyfill()
    let messageReceived: string | null = null

    port.onmessage = (event: MessageEvent<string>): void => {
      messageReceived = event.data
    }

    port.dispatchEvent({ data: 'Hello, world!' } as MessageEvent)
    expect(messageReceived).toBe('Hello, world!')
  })

  it('should handle multiple event listeners', () => {
    const port = new MessagePortPolyfill()
    const messages: Array<string> = []

    const listener1 = ((event: MessageEvent): void => {
      messages.push('Listener1: ' + event.data)
    }) as EventListener
    const listener2 = ((event: MessageEvent): void => {
      messages.push('Listener2: ' + event.data)
    }) as EventListener

    port.addEventListener('message', listener1)
    port.addEventListener('message', listener2)

    port.dispatchEvent({ data: 'Test message' } as MessageEvent)
    expect(messages).toEqual([
      'Listener1: Test message',
      'Listener2: Test message',
    ])
  })

  it('should not invoke removed event listeners', () => {
    const port = new MessagePortPolyfill()
    const messages: Array<string> = []

    const listener = ((event: MessageEvent<string>): void => {
      messages.push(event.data)
    }) as EventListener

    port.addEventListener('message', listener)
    port.dispatchEvent({ data: 'First message' } as MessageEvent)

    port.removeEventListener('message', listener)
    port.dispatchEvent({ data: 'Second message' } as MessageEvent)

    expect(messages).toEqual(['First message'])
  })

  it('should not post messages if otherPort is null', () => {
    const port = new MessagePortPolyfill()
    let messageReceived: string | null = null

    port.onmessage = (event: MessageEvent<string>): void => {
      messageReceived = event.data
    }

    port.postMessage('Hello!')
    expect(messageReceived).toBeNull()
  })
})

describe('MessageChannelPolyfill', () => {
  it('should send and receive messages between ports', () => {
    const channel = new MessageChannelPolyfill()
    const messages: Array<string> = []

    channel.port1.onmessage = (event: MessageEvent<string>): void => {
      messages.push(event.data)
    }

    channel.port2.postMessage('Hello, port1!')
    channel.port2.postMessage('How are you?')

    expect(messages).toEqual(['Hello, port1!', 'How are you?'])
  })

  it('should support addEventListener and removeEventListener', () => {
    const channel = new MessageChannelPolyfill()
    const messages: Array<string> = []

    const listener = ((event: MessageEvent<string>): void => {
      messages.push(event.data)
    }) as EventListener

    channel.port1.addEventListener('message', listener)
    channel.port2.postMessage('Hello, port1!')
    expect(messages).toEqual(['Hello, port1!'])

    channel.port1.removeEventListener('message', listener)
    channel.port2.postMessage('Message after removing listener')

    expect(messages).toEqual(['Hello, port1!'])
  })

  it('should handle bidirectional communication between ports', () => {
    const channel = new MessageChannelPolyfill()
    const messagesPort1: Array<string> = []
    const messagesPort2: Array<string> = []

    channel.port1.onmessage = (event: MessageEvent<string>): void => {
      messagesPort1.push(event.data)
    }

    channel.port2.onmessage = (event: MessageEvent<string>): void => {
      messagesPort2.push(event.data)
    }

    channel.port1.postMessage('Hello, port2!')
    channel.port2.postMessage('Hello, port1!')

    expect(messagesPort1).toEqual(['Hello, port1!'])
    expect(messagesPort2).toEqual(['Hello, port2!'])
  })
})

describe('MessagePort', () => {
  it('should not send a message on close', () => {
    const { port1, port2 } = new MessageChannelPolyfill()

    const messages: Array<string> = []
    port1.onmessage = (event: MessageEvent<string>): void => {
      messages.push(event.data)
    }

    port2.postMessage('Test message')
    expect(messages).toEqual(['Test message'])

    port1.close()
    port2.postMessage('Another message')

    expect(messages).toEqual(['Test message'])
  })
})
