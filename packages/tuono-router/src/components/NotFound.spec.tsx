import type { JSX } from 'react'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { cleanup, render, screen } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent, RouteProps } from '../types'

import { NotFound } from './NotFound'

function createRouteComponent(
  routeType: string,
  RouteComponentFn: (props: RouteProps) => JSX.Element,
): RouteComponent {
  const RootComponent = RouteComponentFn as RouteComponent
  RootComponent.preload = vi.fn()
  RootComponent.displayName = routeType
  return RootComponent
}

const root = new Route({
  isRoot: true,
  component: createRouteComponent('root', ({ children }) => (
    <div data-testid="root">{children}</div>
  )),
})

const pushMock = vi.fn()
const replaceMock = vi.fn()

// Mock required for the <Link /> component
vi.mock('../hooks/useRouter', () => ({
  useRouter: (): { push: typeof pushMock; replace: typeof replaceMock } => ({
    push: pushMock,
    replace: replaceMock,
  }),
}))

vi.mock('../hooks/useServerPayloadData.ts', () => ({
  useServerPayloadData: (): { data: unknown; isLoading: boolean } => {
    return {
      data: undefined,
      isLoading: false,
    }
  },
}))

const { useRouterContext } = vi.hoisted(() => {
  return { useRouterContext: vi.fn() }
})

vi.mock('../components/RouterContext', () => ({
  useRouterContext,
}))

describe('test <NotFound /> component', () => {
  afterEach(cleanup)

  describe('when a custom 404 page exists', () => {
    it('should render the custom 404 page', () => {
      useRouterContext.mockReturnValue({
        router: {
          routesById: {
            '/404': new Route({
              getParentRoute: (): Route => root,
              component: createRouteComponent('404', () => (
                <div data-testid="404">custom 404</div>
              )),
            }),
            __root__: root,
          },
        },
      })
      render(<NotFound />)
      expect(screen.getByTestId('root')).toMatchInlineSnapshot(
        `
        <div
          data-testid="root"
        >
          <div
            data-testid="404"
          >
            custom 404
          </div>
        </div>
        `,
      )
    })
  })

  describe('when a custom 404 page does not exist', () => {
    it('should render the default 404 page wrapped by the user defined __layout', () => {
      useRouterContext.mockReturnValue({
        router: {
          routesById: {
            __root__: root,
          },
        },
      })
      render(<NotFound />)
      expect(screen.getByTestId('root')).toMatchInlineSnapshot(
        `
        <div
          data-testid="root"
        >
          <h1>
            404 Not found
          </h1>
          <a
            href="/"
          >
            Return home
          </a>
        </div>
        `,
      )
    })
  })
})
