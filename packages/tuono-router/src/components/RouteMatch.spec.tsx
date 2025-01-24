import type { JSX } from 'react'
import { afterEach, describe, expect, test, vi } from 'vitest'

import { cleanup, render, screen } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent, RouteProps } from '../types'

import { RouteMatch } from './RouteMatch'

function createRouteComponent(
  routeType: string,
  cose: (props: RouteProps) => JSX.Element,
): RouteComponent {
  const RootComponent = cose as RouteComponent
  RootComponent.preload = vi.fn()
  RootComponent.displayName = routeType
  return RootComponent
}

const root = new Route({
  isRoot: true,
  component: createRouteComponent('root', ({ children }) => (
    <div data-testid="root">root route {children}</div>
  )),
})

const parent = new Route({
  component: createRouteComponent('parent', ({ children }) => (
    <div data-testid="parent">parent route {children}</div>
  )),
  getParentRoute: (): Route => root,
})

const route = new Route({
  component: createRouteComponent('route', () => (
    <p data-testid="route">current route</p>
  )),
  getParentRoute: (): Route => parent,
})

vi.mock('../hooks/useServerPayloadData.ts', () => ({
  useServerPayloadData: (): { data: unknown; isLoading: boolean } => {
    return {
      data: undefined,
      isLoading: false,
    }
  },
}))

describe('Test RouteMatch component', () => {
  afterEach(cleanup)

  test('It should correctly render nested routes', () => {
    render(<RouteMatch route={route} serverInitialData={{}} />)

    expect(screen.getByTestId('root')).toMatchInlineSnapshot(
      `
      <div
        data-testid="root"
      >
        root route 
        <div
          data-testid="parent"
        >
          parent route 
          <p
            data-testid="route"
          >
            current route
          </p>
        </div>
      </div>
    `,
    )
    expect(screen.getByTestId('route')).toMatchInlineSnapshot(`
      <p
        data-testid="route"
      >
        current route
      </p>
    `)
  })
})
