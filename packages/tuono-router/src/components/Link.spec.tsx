import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, fireEvent, screen, cleanup } from '@testing-library/react'

import { Link } from './Link'

const pushMock = vi.fn()
const replaceMock = vi.fn()
const preloadMock = vi.fn()

vi.mock('../hooks/useRouter', () => ({
  useRouter: (): { push: typeof pushMock; replace: typeof replaceMock } => ({
    push: pushMock,
    replace: replaceMock,
  }),
}))

vi.mock('../hooks/useRoute', () => ({
  useRoute: (): { component: { preload: typeof preloadMock } } => ({
    component: { preload: preloadMock },
  }),
}))

let intersectionObserverCallback: ((inView: boolean) => void) | undefined

vi.mock('react-intersection-observer', () => ({
  useInView: (options: {
    onChange: (inView: boolean) => void
  }): {
    ref: () => void
  } => {
    intersectionObserverCallback = options.onChange
    return { ref: vi.fn() }
  },
}))

describe('<Link />', () => {
  beforeEach(() => {
    cleanup()
    pushMock.mockReset()
    preloadMock.mockReset()
    intersectionObserverCallback = undefined
  })

  it('renders with correct href and text', () => {
    render(<Link href="/test">Test Link</Link>)
    const link = screen.getByRole('link', { name: 'Test Link' })

    expect(link.getAttribute('href')).toBe('/test')
  })

  it('calls router.push on normal click', () => {
    render(<Link href="/test">Test Link</Link>)
    const link = screen.getByRole('link')

    fireEvent.click(link)
    expect(pushMock).toHaveBeenCalledWith('/test', { scroll: true })
  })

  it('calls router.replace on click when the replace prop is true', () => {
    render(
      <Link href="/test" replace>
        Test Link
      </Link>,
    )
    const link = screen.getByRole('link')

    fireEvent.click(link)
    expect(replaceMock).toHaveBeenCalledWith('/test', { scroll: true })
    expect(pushMock).not.toHaveBeenCalled()
  })

  it('does not navigate if href starts with "#"', () => {
    render(<Link href="#section">Anchor Link</Link>)
    const link = screen.getByRole('link')

    fireEvent.click(link)
    expect(pushMock).not.toHaveBeenCalled()
  })

  it('preloads route when in viewport and preload is true', () => {
    render(
      <Link href="/test" preload={true}>
        Test Link
      </Link>,
    )

    intersectionObserverCallback?.(true)
    expect(preloadMock).toHaveBeenCalled()
  })

  it('does not preload route when preload is false', () => {
    render(
      <Link href="/test" preload={false}>
        Test Link
      </Link>,
    )

    intersectionObserverCallback?.(true)
    expect(preloadMock).not.toHaveBeenCalled()
  })

  it('does not call router.push when clicked with a modifier key', () => {
    render(<Link href="/test">Test Link</Link>)
    const link = screen.getByRole('link')

    fireEvent.click(link, { ctrlKey: true })
    fireEvent.click(link, { metaKey: true })
    fireEvent.click(link, { shiftKey: true })
    fireEvent.click(link, { altKey: true })

    expect(pushMock).not.toHaveBeenCalled()
  })

  it('calls onClick handler when clicked', () => {
    const onClickMock = vi.fn()
    render(
      <Link href="/test" onClick={onClickMock}>
        Test Link
      </Link>,
    )
    const link = screen.getByRole('link')

    fireEvent.click(link)

    expect(onClickMock).toHaveBeenCalledTimes(1)
    expect(pushMock).toHaveBeenCalledWith('/test', { scroll: true })
  })

  it('calls onClick but does not navigate when clicked with a modifier key', () => {
    const onClickMock = vi.fn()
    render(
      <Link href="/test" onClick={onClickMock}>
        Test Link
      </Link>,
    )
    const link = screen.getByRole('link')

    fireEvent.click(link, { ctrlKey: true })
    fireEvent.click(link, { metaKey: true })
    fireEvent.click(link, { shiftKey: true })
    fireEvent.click(link, { altKey: true })

    expect(onClickMock).toHaveBeenCalledTimes(4)
    expect(pushMock).not.toHaveBeenCalled()
  })
})
