import React from 'react'
import { hydrateRoot } from 'react-dom/client'
import { RouterProvider, createRouter } from 'tuono-router'
import type { createRoute } from 'tuono-router'

type RouteTree = ReturnType<typeof createRoute>

export function hydrate(routeTree: RouteTree): void {
  // Create a new router instance
  const router = createRouter({ routeTree })

  // eslint-disable-next-line
  const rootElement = document.getElementById('__tuono')!

  hydrateRoot(
    rootElement,
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>,
  )
}
