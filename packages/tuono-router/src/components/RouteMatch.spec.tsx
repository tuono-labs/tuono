import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render, screen } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent, RouteProps } from '../types'
import { useServerPayloadData } from '../hooks/useServerPayloadData'

import { RouteMatch } from './RouteMatch'

function createRouteComponent(routeType: string): RouteComponent {
  const RootComponent = (({ children }: RouteProps) => (
    <div data-testid={routeType}>
      {`${routeType} route`}
      {children}
    </div>
  )) as RouteComponent
  RootComponent.preload = vi.fn()
  RootComponent.displayName = routeType
  return RootComponent
}

function createLeafRouteComponent(routeType: string): RouteComponent {
  const RootComponent = (({ data }: RouteProps) => (
    <div data-testid={routeType}>{data ? JSON.stringify(data) : null}</div>
  )) as RouteComponent
  RootComponent.preload = vi.fn()
  RootComponent.displayName = routeType
  return RootComponent
}

const root = new Route({
  isRoot: true,
  component: createRouteComponent('root'),
})

const parent = new Route({
  component: createRouteComponent('parent'),
  getParentRoute: (): Route => root,
})

const route = new Route({
  component: createLeafRouteComponent('current'),
  getParentRoute: (): Route => parent,
})

vi.mock('../hooks/useServerPayloadData', () => ({
  useServerPayloadData: vi.fn(),
}))

const useServerPayloadDataMock = vi.mocked(useServerPayloadData)

describe('<RouteMatch />', () => {
  afterEach(cleanup)

  it('should correctly render nested routes', () => {
    useServerPayloadDataMock.mockReturnValue({
      data: { some: 'data' },
      isLoading: false,
    })

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
            {"some":"data"}
          </div>
        </div>
      </div>
    `,
    )
  })

  it('should return null data when while loading', () => {
    useServerPayloadDataMock.mockReturnValue({
      data: { some: 'data' },
      isLoading: true,
    })

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
          />
        </div>
      </div>
    `,
    )
  })
})
