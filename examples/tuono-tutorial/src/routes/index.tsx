// src/routes/index.tsx
import type { JSX } from 'react'
import { type TuonoProps } from 'tuono'

import PokemonLink from '@/components/PokemonLink'

interface Pokemon {
  name: string
}

interface IndexProps {
  results: Array<Pokemon>
}

export default function IndexPage({
  data,
}: TuonoProps<IndexProps>): JSX.Element | null {
  if (!data?.results) return null
  return (
    <>
      <header className="header">
        <a href="https://crates.io/crates/tuono" target="_blank">
          Crates
        </a>
        <a href="https://www.npmjs.com/package/tuono" target="_blank">
          Npm
        </a>
      </header>
      <div className="title-wrap">
        <h1 className="title">
          TU<span>O</span>NO
        </h1>
        <div className="logo">
          <img src="rust.svg" className="rust" />
          <img src="react.svg" className="react" />
        </div>
      </div>
      <ul style={{ flexWrap: 'wrap', display: 'flex', gap: 10 }}>
        <PokemonLink pokemon={{ name: 'GOAT' }} id={0} />
        {data.results.map((pokemon, i) => {
          return <PokemonLink pokemon={pokemon} id={i + 1} key={i} />
        })}
      </ul>
    </>
  )
}
