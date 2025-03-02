import fs from 'node:fs/promises'

import { beforeEach, describe, expect, it, vitest } from 'vitest'
import react from '@vitejs/plugin-react-swc'

import { createJsonConfig } from './create-json-config'

const writeFileSpy = vitest.spyOn(fs, 'writeFile').mockResolvedValue(void 0)

describe('createJsonConfig', () => {
  beforeEach(() => {
    writeFileSpy.mockClear()
  })

  it('should process config with only server property', async () => {
    const sampleConfig = { server: { host: 'h', origin: null, port: 1 } }

    await createJsonConfig(sampleConfig)

    expect(writeFileSpy).toHaveBeenCalledWith(
      expect.any(String),
      expect.stringContaining(JSON.stringify(sampleConfig)),
      expect.any(String),
    )
  })

  it('should process config with plugins', async () => {
    const sampleConfig = { server: { host: 'h', origin: null, port: 1 } }

    await createJsonConfig({ ...sampleConfig, vite: { plugins: [react()] } })

    expect(writeFileSpy).toHaveBeenCalledWith(
      expect.any(String),
      expect.stringContaining(JSON.stringify(sampleConfig)),
      expect.any(String),
    )
  })

  it('should process config with only server property including origin', async () => {
    const sampleConfig = { server: { host: 'h', origin: 'o', port: 1 } }

    await createJsonConfig(sampleConfig)

    expect(writeFileSpy).toHaveBeenCalledWith(
      expect.any(String),
      expect.stringContaining(JSON.stringify(sampleConfig)),
      expect.any(String),
    )
  })
})
