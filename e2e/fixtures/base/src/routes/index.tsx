import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'
import { Link } from 'tuono'

interface IndexProps {
  subtitle: string
}

export default function IndexPage({
  data,
  isLoading,
}: TuonoProps<IndexProps>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>
  }

  return (
    <>
      <h1>TUONO</h1>
      <h2>{data?.subtitle}</h2>
      <Link href={'/second-route'}>Routing link</Link>)
    </>
  )
}
