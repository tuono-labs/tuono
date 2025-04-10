import type { JSX } from 'react'
import type { TuonoRouteProps } from 'tuono'
import { Link } from 'tuono'
import type { MyResponse } from 'tuono/types'

export default function IndexPage({
  data,
  isLoading,
}: TuonoRouteProps<MyResponse>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>
  }

  return (
    <>
      <h1>TUONO</h1>
      <h2>{data.subtitle}</h2>
      <Link href={'/second-route'}>Routing link</Link>
    </>
  )
}
