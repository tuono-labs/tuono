import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render, screen } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent, RouteProps } from '../types'
import { useServerPayloadData } from '../hooks/useServerPayloadData'

import { RouteMatch } from './RouteMatch'

function createRouteComponent(
  routeType: string,
  includeChildren: boolean,
): RouteComponent {
  const RootComponent = (({ children }: RouteProps) => (
    <div data-testid={routeType}>
      {`${routeType} route`}
      {includeChildren ? children : null}
    </div>
  )) as RouteComponent
  RootComponent.preload = vi.fn()
  RootComponent.displayName = routeType
  return RootComponent
}

const root = new Route({
  isRoot: true,
  component: createRouteComponent('root', true),
})

const parent = new Route({
  component: createRouteComponent('parent', true),
  getParentRoute: (): Route => root,
})

const route = new Route({
  component: createRouteComponent('current', false),
  getParentRoute: (): Route => parent,
})

vi.mock('../hooks/useServerPayloadData', () => ({
  useServerPayloadData: vi.fn(),
}))
vi.mocked(useServerPayloadData).mockReturnValue({
  data: undefined,
  isLoading: false,
})

describe('<RouteMatch />', () => {
  afterEach(cleanup)

  it('should correctly render nested routes', () => {
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
          <div
            data-testid="current"
          >
            current route
          </div>
        </div>
      </div>
    `,
    )
  })
})
