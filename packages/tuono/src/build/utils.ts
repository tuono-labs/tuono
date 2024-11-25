import { TuonoConfig } from '../config'

export const loadConfig = async (): Promise<TuonoConfig> => {
	try {
		return await import(`${process.cwd()}/.tuono/config/config.js`)
	} catch {
		return {}
	}
}

export const blockingAsync = (callback: () => Promise<void>) => {
	; (async () => {
		await callback()
	})()
}
