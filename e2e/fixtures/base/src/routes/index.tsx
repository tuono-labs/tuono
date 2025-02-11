import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

interface IndexProps {
  subtitle: string
}

export default function IndexPage({
  isLoading,
}: TuonoProps<IndexProps>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>
  }

  return (
    <div className="title-wrap">
      <h1 className="title">
        TU<span>O</span>NO
      </h1>
      <div className="logo">
        <img src="rust.svg" className="rust" />
        <img src="react.svg" className="react" />
      </div>
    </div>
  )
}
