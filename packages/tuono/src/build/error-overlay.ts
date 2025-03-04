/**
 * Most of this file is takend from the Vite project.
 * We needed to re-export it in order to apply the necessary changes related to
 * tuono.
 *
 * Source: https://github.com/vitejs/vite/blob/2c51565ec044904a080ef5649034c37f02212c7b/packages/vite/src/client/overlay.ts#L209
 * License: https://github.com/vitejs/vite/blob/main/LICENSE
 */
import type { ErrorPayload, Plugin } from 'vite'

// set :host styles to make playwright detect the element as visible
const templateStyle = /*css*/ `
:host {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 99999;
  --monospace: 'SFMono-Regular', Consolas,
  'Liberation Mono', Menlo, Courier, monospace;
  --red: #ff5555;
  --yellow: #e2aa53;
  --purple: #cfa4ff;
  --cyan: #2dd9da;
  --dim: #c9c9c9;

  --window-background: #181818;
  --window-color: #d8d8d8;
}

.backdrop {
  position: fixed;
  z-index: 99999;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow-y: scroll;
  margin: 0;
  background: rgba(0, 0, 0, 0.66);
}

.window {
  font-family: var(--monospace);
  line-height: 1.5;
  max-width: 80vw;
  color: var(--window-color);
  box-sizing: border-box;
  margin: 30px auto;
  padding: 2.5vh 4vw;
  position: relative;
  background: var(--window-background);
  border-radius: 6px 6px 8px 8px;
  box-shadow: 0 19px 38px rgba(0,0,0,0.30), 0 15px 12px rgba(0,0,0,0.22);
  overflow: hidden;
  border-top: 8px solid var(--red);
  direction: ltr;
  text-align: left;
}

pre {
  font-family: var(--monospace);
  font-size: 16px;
  margin-top: 0;
  overflow-x: scroll;
  scrollbar-width: none;
}

pre::-webkit-scrollbar {
  display: none;
}

pre.frame::-webkit-scrollbar {
  display: block;
  height: 5px;
}

pre.frame::-webkit-scrollbar-thumb {
  background: #999;
  border-radius: 5px;
}

pre.frame {
  scrollbar-width: thin;
}

.message {
  line-height: 1.3;
  font-weight: 600;
  white-space: pre-wrap;
}

.message-body {
  color: var(--red);
}

.plugin {
  color: var(--purple);
}

.file {
  color: var(--cyan);
  margin-bottom: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.frame {
  color: var(--yellow);
}

.stack {
  font-size: 13px;
  color: var(--dim);
}

.tip {
  font-size: 13px;
  color: #999;
  border-top: 1px dotted #999;
  padding-top: 13px;
  line-height: 1.8;
}

code {
  font-size: 13px;
  font-family: var(--monospace);
  color: var(--yellow);
}

.file-link {
  text-decoration: underline;
  cursor: pointer;
}

kbd {
  line-height: 1.5;
  font-family: ui-monospace, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 0.75rem;
  font-weight: 700;
  background-color: rgb(38, 40, 44);
  color: rgb(166, 167, 171);
  padding: 0.15rem 0.3rem;
  border-radius: 0.25rem;
  border-width: 0.0625rem 0.0625rem 0.1875rem;
  border-style: solid;
  border-color: rgb(54, 57, 64);
  border-image: initial;
}
`

const fileRE = /(?:[a-zA-Z]:\\|\/).*?:\d+:\d+/g
const codeframeRE = /^(?:>?\s*\d+\s+\|.*|\s+\|\s*\^.*)\r?\n/gm

const overlayTemplate = `
<div class="backdrop" part="backdrop">
  <div class="window" part="window">
    <pre class="message" part="message">
      <span class="plugin" part="plugin"></span>
      <span class="message-body" part="message-body"></span>
    </pre>
    <pre class="file" part="file">
    </pre>
    <pre class="frame" part="frame"></pre>
    <pre class="stack" part="stack"></pre>
    <div class="tip" part="tip">Click outside, press <kbd>Esc</kbd> key, or fix the code to dismiss.</div>
  </div>
  <style>${templateStyle}</style>
</div>
`

const HTMLElement: typeof globalThis.HTMLElement =
  // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
  globalThis.HTMLElement ??
  // eslint-disable-next-line @typescript-eslint/no-extraneous-class
  class {}
export class ErrorOverlay extends HTMLElement {
  /**
   * All the fields need to be implemented in the constructor otherwise
   * we will get an error when trying to use the class.
   */

  constructor(err: ErrorPayload['err'], links = true) {
    super()
    // @ts-expect-error cannot declare outside prop
    this.root = this.attachShadow({ mode: 'open' })

    const root = this.getRoot()

    root.innerHTML = overlayTemplate

    codeframeRE.lastIndex = 0
    const hasFrame = err.frame && codeframeRE.test(err.frame)
    const message = hasFrame
      ? err.message.replace(codeframeRE, '')
      : err.message

    if (err.plugin) {
      this.text('.plugin', `[plugin:${err.plugin}] `)
    }
    this.text('.message-body', message.trim())

    const [file] = (err.loc?.file || err.id || 'unknown file').split(`?`)
    if (err.loc && file) {
      this.text('.file', `${file}:${err.loc.line}:${err.loc.column}`, links)
    } else if (err.id && file) {
      this.text('.file', file)
    }

    if (hasFrame && err.frame) {
      this.text('.frame', err.frame.trim())
    }
    this.text('.stack', err.stack, links)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    root.querySelector('.window')!.addEventListener('click', (e: Event) => {
      e.stopPropagation()
    })

    this.addEventListener('click', () => {
      this.close()
    })

    // @ts-expect-error cannot declare outside prop
    this.closeOnEsc = (e: KeyboardEvent): void => {
      if (e.key === 'Escape' || e.code === 'Escape') {
        this.close()
      }
    }

    const closeOnEsc = this.getCloseOnEsc()

    document.addEventListener('keydown', closeOnEsc)
  }

  getRoot(): ShadowRoot {
    // @ts-expect-error cannot declare outside prop
    return this.root as unknown as ShadowRoot
  }

  getCloseOnEsc(): (e: KeyboardEvent) => void {
    // @ts-expect-error cannot declare outside prop
    return this.closeOnEsc as unknown as (e: KeyboardEvent) => void
  }

  text(selector: string, text: string, linkFiles = false): void {
    const root = this.getRoot()

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const el = root.querySelector(selector)!
    if (!linkFiles) {
      el.textContent = text
    } else {
      let curIndex = 0
      let match: RegExpExecArray | null
      fileRE.lastIndex = 0
      while ((match = fileRE.exec(text))) {
        const { 0: file, index } = match
        const frag = text.slice(curIndex, index)
        el.appendChild(document.createTextNode(frag))
        const link = document.createElement('a')
        link.textContent = file
        link.className = 'file-link'
        el.appendChild(link)
        curIndex += frag.length + file.length
      }
    }
  }
  close(): void {
    const closeOnEsc = this.getCloseOnEsc()
    this.parentNode?.removeChild(this)
    document.removeEventListener('keydown', closeOnEsc)
  }
}

function getOverlayCode(): string {
  return `
		const overlayTemplate = \`${overlayTemplate}\`;
		${ErrorOverlay.toString()}
	`
}

function patchOverlay(code: string): string {
  return code.replace(
    'class ErrorOverlay',
    getOverlayCode() + '\nclass ViteErrorOverlay',
  )
}

export const ErrorOverlayVitePlugin: Plugin = {
  name: 'tuono-error-overlay-plugin',
  transform(code, id, opts = {}) {
    if (opts.ssr) return
    if (!id.includes('vite/dist/client/client.mjs')) return

    return patchOverlay(code)
  },
}
