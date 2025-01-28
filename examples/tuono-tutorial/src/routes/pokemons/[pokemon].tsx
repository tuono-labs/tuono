import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'
import { Link } from 'tuono'

import { Pokemon } from '../../models/Pokemon'
import PokemonView from '../../components/PokemonView'

export default function PokemonPage({
  isLoading,
  data,
}: TuonoProps<Pokemon>): JSX.Element {
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
