import type { ReactNode } from 'react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render } from '@testing-library/react'

import { Route } from '../route'
import type { RouteComponent } from '../types'
import type { RouterInstanceType } from '../router'

import { NotFound } from './NotFound'
import { RouteMatch } from './RouteMatch'
import { useRouterContext } from './RouterContext'
import { NotFoundDefaultContent } from './NotFoundDefaultContent'

vi.mock('../components/RouterContext', () => ({
  useRouterContext: vi.fn(),
}))
vi.mock('./RouteMatch', () => ({
  RouteMatch: vi.fn(),
}))
vi.mock('./NotFoundDefaultContent', () => ({
  NotFoundDefaultContent: vi.fn(),
}))

interface RouterMock {
  router: Pick<RouterInstanceType, 'routesById'>
}
const useRouterContextMock = vi.mocked(useRouterContext as () => RouterMock)
const RouteMatchMock = vi.mocked(RouteMatch)
const NotFoundDefaultContentMock = vi.mocked(NotFoundDefaultContent)

describe('<NotFound />', () => {
  afterEach(() => {
    cleanup()
    vi.resetAllMocks()
  })

  const root = new Route({
    isRoot: true,
    component: (({ children }: { children: ReactNode }) => (
      <div>{children}</div>
    )) as unknown as RouteComponent,
  })

  describe('when a custom 404 page exists', () => {
    it('should render the custom 404 page', () => {
      const customRoute404 = new Route({
        getParentRoute: (): Route => root,
        component: vi.fn() as unknown as RouteComponent,
      })

      useRouterContextMock.mockReturnValue({
        router: {
          routesById: {
            '/404': customRoute404,
            __root__: root,
          },
        },
      })

      render(<NotFound />)

      expect(RouteMatchMock).toHaveBeenCalledExactlyOnceWith(
        { route: customRoute404, serverInitialData: {} },
        undefined, // deprecated react context parameter
      )
      expect(NotFoundDefaultContentMock).not.toHaveBeenCalled()
    })
  })

  describe('when a custom 404 page does not exist', () => {
    it('should render the default 404 page, wrapped by the root __layout', () => {
      useRouterContextMock.mockReturnValue({
        router: {
          routesById: {
            __root__: root,
          },
        },
      })

      render(<NotFound />)

      expect(RouteMatchMock).not.toHaveBeenCalled()
      expect(NotFoundDefaultContentMock).toHaveBeenCalledOnce()
    })
  })
})
