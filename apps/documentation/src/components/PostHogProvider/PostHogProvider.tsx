import type { JSX, ReactNode } from 'react'
import { useEffect } from 'react'
import { PostHogProvider as PostHogLibraryProvider } from 'posthog-js/react'
import posthog from 'posthog-js'

interface PostHogProviderProps {
  children: ReactNode
}

export default function PostHogProvider({
  children,
}: PostHogProviderProps): JSX.Element {
  useEffect(() => {
    if (import.meta.env.VITE_ENABLE_POSTHOG === 'true') {
      posthog?.init(import.meta.env.VITE_PUBLIC_POSTHOG_KEY || '', {
        api_host:
          import.meta.env.VITE_PUBLIC_POSTHOG_HOST ||
          'https://eu.i.posthog.com',
        persistence: 'memory', // Cookieless tracking
        loaded: (ph) => {
          if (import.meta.env.VITE_ENV === 'development') ph.debug()
        },
      })
    }
  }, [])

  return (
    <PostHogLibraryProvider client={posthog}>{children}</PostHogLibraryProvider>
  )
}
