import { useEffect } from 'react'
import { usePostHog } from 'posthog-js/react'
import { useRouter } from 'tuono'

export default function PostHogPageView() {
  const { pathname } = useRouter()
  const posthog = usePostHog()

  // Track pageviews
  useEffect(() => {
    if (pathname && posthog) {
      let url = window.origin + pathname

      posthog.capture('$pageview', { $current_url: url })
    }
  }, [pathname, posthog])

  return null
}
