import type * as React from 'react'
import { afterEach, describe, expect, test, vi } from 'vitest'

import { cleanup, render, screen } from '@testing-library/react'

import type { Route } from '../route'

import { RouteMatch } from './RouteMatch'

import '@testing-library/jest-dom'

interface Props {
  children: React.ReactNode
}

const root = {
  isRoot: true,
  component: ({ children }: Props) => (
    <div data-testid="root">root route {children}</div>
  ),
} as unknown as Route

const parent = {
  component: ({ children }: Props) => (
    <div data-testid="parent">parent route {children}</div>
  ),
  options: {
    getParentRoute: () => root,
  },
} as unknown as Route

const route = {
  component: () => <p data-testid="route">current route</p>,
  options: {
    getParentRoute: () => parent,
  },
} as unknown as Route

describe('Test RouteMatch component', () => {
  afterEach(() => {
    cleanup()
  })

  test('It should correctly render nested routes', () => {
    vi.mock('../hooks/useServerPayloadData.ts', () => ({
      useServerPayloadData: (): { data: unknown; isLoading: boolean } => {
        return {
          data: undefined,
          isLoading: false,
        }
      },
    }))

    render(<RouteMatch route={route} serverInitialData={{}} />)
    expect(screen.getByTestId('root')).toHaveTextContent(
      'root route parent route current route',
    )
    expect(screen.getByTestId('route')).toHaveTextContent('current route')
  })
})
