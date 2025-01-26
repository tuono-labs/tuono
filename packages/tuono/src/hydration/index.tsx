import { hydrateRoot } from 'react-dom/client'
import { createRouter } from 'tuono-router'
import type { createRoute } from 'tuono-router'

import { TuonoEntryPoint } from '../shared/TuonoEntryPoint'

type RouteTree = ReturnType<typeof createRoute>

export function hydrate(routeTree: RouteTree): void {
  // Create a new router instance
  const router = createRouter({ routeTree })

  hydrateRoot(document, <TuonoEntryPoint router={router} />)
}
