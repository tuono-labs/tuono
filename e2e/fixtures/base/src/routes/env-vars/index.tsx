import type { JSX } from 'react'
import type { TuonoRouteProps } from 'tuono'
import type { EnvVarsResponse } from 'tuono/types'

export default function EnvVarsPage({
  data,
  isLoading,
}: TuonoRouteProps<EnvVarsResponse>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>
  }

  return (
    <>
      <h1>Env Vars</h1>
      <p data-testid="server-var">{data.server_var}</p>
      <p data-testid="public-var-server">{data.public_var}</p>
      <p data-testid="public-var-client">
        {import.meta.env.TUONO_PUBLIC_TEST_VAR}
      </p>
    </>
  )
}
