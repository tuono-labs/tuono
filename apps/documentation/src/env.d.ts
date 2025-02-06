interface ImportMetaEnv {
  readonly VITE_PUBLIC_POSTHOG_KEY: string
  readonly VITE_PUBLIC_POSTHOG_HOST: string
  readonly VITE_ENV: 'production' | 'development'
  readonly VITE_ENABLE_POSTHOG: 'true' | 'false'
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
