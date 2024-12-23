import React from 'react'

import type { Router } from '../router'

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const routerContext = React.createContext<Router>(null!)

const TUONO_CONTEXT_GLOBAL_NAME = '__TUONO_CONTEXT__'

export function getRouterContext(): React.Context<Router> {
  if (typeof document === 'undefined') {
    return routerContext
  }

  if (window[TUONO_CONTEXT_GLOBAL_NAME]) {
    return window[TUONO_CONTEXT_GLOBAL_NAME]
  }

  window[TUONO_CONTEXT_GLOBAL_NAME] = routerContext

  return routerContext
}
