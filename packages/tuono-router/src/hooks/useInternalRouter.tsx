import * as React from 'react'

import { getRouterContext } from '../components/RouterContext'
import type { Router } from '../router'

export function useInternalRouter(): Router {
  return React.useContext(getRouterContext())
}
