import { useEffect } from 'react'
import { usePostHog } from 'posthog-js/react'
import { useRouter } from 'tuono'

export default function PostHogPageView(): null {
  const { pathname } = useRouter()
  const posthog = usePostHog()

  // Track pageviews
  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    if (pathname && posthog) {
      const url = window.origin + pathname

      posthog.capture('$pageview', { $current_url: url })
    }
  }, [pathname, posthog])

  return null
}
