import { useId, type JSX } from 'react'

export default function IndexPage(): JSX.Element {
  const id = useId()

  return (
    <>
      <h1>{id}</h1>
    </>
  )
}
