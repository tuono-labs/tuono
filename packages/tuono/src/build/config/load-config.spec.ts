import { describe, expect, it, vi } from 'vitest'

import { loadConfig } from './load-config'

describe('loadConfig', () => {
  it('should error if the config does not exist', async () => {
    const consoleErrorSpy = vi
      .spyOn(console, 'error')
      .mockImplementation(() => undefined)
    await loadConfig()

    expect(consoleErrorSpy).toHaveBeenCalledTimes(2)
  })
})
