import { useMemo } from 'react'
import type { JSX } from 'react'

interface PokemonImageProps {
  id: number

  size: 'sm' | 'md'
}

export function PokemonImage({ id, size }: PokemonImageProps): JSX.Element {
  const src = useMemo(() => {
    const baseURL =
      'https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon'

    switch (size) {
      case 'md':
        return `${baseURL}/other/official-artwork/${id}.png`

      case 'sm':
      default:
        return `${baseURL}/${id}.png`
    }
  }, [id, size])

  return <img src={src} alt="" />
}
