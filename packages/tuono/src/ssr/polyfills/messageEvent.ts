function Event(type) {
	this.type = type
	this.target = undefined
}

export function MessageEventPolyfill(type, options) {
	Event.call(this, type)
	this.data = options.data
	this.lastEventId = options.lastEventId
}

MessageEvent.prototype = Object.create(Event.prototype)
