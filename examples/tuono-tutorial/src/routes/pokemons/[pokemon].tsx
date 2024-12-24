import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

import PokemonView from '@/components/PokemonView'

interface Pokemon {
  name: string
  id: string
  weight: number
  height: number
}

export default function PokemonPage({
  data,
}: TuonoProps<Pokemon>): JSX.Element {
  return (
    <>
      <title>{`Pokemon: ${data?.name ?? ''}`}</title>

      <PokemonView pokemon={data} />
    </>
  )
}
