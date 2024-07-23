import * as React from 'react'
import { afterEach, describe, expect, test, vi } from 'vitest'
import { RouteMatch } from './RouteMatch'
import { cleanup, render, screen } from '@testing-library/react'
import type { Route } from '../route'
import '@testing-library/jest-dom'

interface Props {
  children: React.ReactNode
}

const root = {
  isRoot: true,
  component: ({ children }: Props) => (
    <div data-testid="root">root route {children}</div>
  ),
} as Route

const parent = {
  component: ({ children }: Props) => (
    <div data-testid="parent">parent route {children}</div>
  ),
  options: {
    getParentRoute: () => root,
  },
} as Route

const route = {
  component: () => <p data-testid="route">current route</p>,
  options: {
    getParentRoute: () => parent,
  },
} as Route

describe('Test RouteMatch component', () => {
  afterEach(() => {
    cleanup()
  })

  test('It should correctly render nested routes', () => {
    vi.mock('../hooks/useServerSideProps.tsx', () => ({
      useServerSideProps: (): { data: any; isLoading: boolean } => {
        return {
          data: undefined,
          isLoading: false,
        }
      },
    }))

    render(<RouteMatch route={route} serverSideProps={{}} />)
    expect(screen.getByTestId('root')).toHaveTextContent(
      'root route parent route current route',
    )
    expect(screen.getByTestId('route')).toHaveTextContent('current route')
  })
})
