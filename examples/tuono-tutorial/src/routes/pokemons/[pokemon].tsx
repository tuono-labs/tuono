// src/routes/pokemons/[pokemon].tsx
import type { JSX } from 'react'
import type { TuonoRouteProps } from 'tuono'
import { Link } from 'tuono'

import PokemonView from '../../components/PokemonView'

interface Pokemon {
  id: number
  name: string
  weight: number
  height: number
}

export default function PokemonPage({
  isLoading,
  data,
}: TuonoRouteProps<Pokemon>): JSX.Element {
  return (
    <div>
      <Link href="/">Back</Link>

      {isLoading && (
        <>
          <title>Pokemon: loading...</title>
          <div>Loading...</div>
        </>
      )}

      {data?.id && (
        <>
          <title>{`Pokemon: ${data.name}`}</title>
          <PokemonView pokemon={data} />
        </>
      )}
    </div>
  )
}
