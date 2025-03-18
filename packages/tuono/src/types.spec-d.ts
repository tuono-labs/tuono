import { describe, it, expectTypeOf } from 'vitest'

import type { TuonoRouteProps } from './types'

describe('TuonoRouteProps', () => {
  interface MyData {
    something: string
  }

  type RouteProps = TuonoRouteProps<MyData>

  it('should have correct union types', () => {
    expectTypeOf<RouteProps>()
      .toHaveProperty('isLoading')
      .toEqualTypeOf<boolean>()

    expectTypeOf<RouteProps>()
      .toHaveProperty('data')
      .toEqualTypeOf<null | MyData>()
  })

  it('should correctly infer `data` based upon `isLoading`', () => {
    expectTypeOf<RouteProps>()
      .extract<{ isLoading: true }>()
      .toHaveProperty('data')
      .toEqualTypeOf<null>()

    expectTypeOf<RouteProps>()
      .extract<{ isLoading: false }>()
      .toHaveProperty('data')
      .toEqualTypeOf<MyData>()
  })
})
