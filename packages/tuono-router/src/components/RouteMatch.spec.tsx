import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render, screen } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent, RouteProps } from '../types'
import { useServerPayloadData } from '../hooks/useServerPayloadData'
import { useRouterContext } from '../components/RouterContext'

import { RouteMatch } from './RouteMatch'
import { Router } from '../router'

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

vi.mock('../components/RouterContext', () => ({
  useRouterContext: vi.fn(),
}))

const useServerPayloadDataMock = vi.mocked(useServerPayloadData)
const useRouterContextMock = vi.mocked(useRouterContext)

describe('<RouteMatch />', () => {
  afterEach(cleanup)

  it('should correctly render nested routes', () => {
    useServerPayloadDataMock.mockReturnValue({
      data: { some: 'data' },
    })

    // @ts-expect-error no need to define the full context here
    useRouterContextMock.mockReturnValue({
      isTransitioning: false,
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
    })

    // @ts-expect-error no need to define the full context here
    useRouterContextMock.mockReturnValue({
      isTransitioning: true,
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
