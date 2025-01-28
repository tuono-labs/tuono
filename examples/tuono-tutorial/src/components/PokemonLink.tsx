import type { JSX } from 'react'
import { Link } from 'tuono'

import { PokemonImage } from './PokemonImage'

import styles from './PokemonLink.module.css'

interface PokemonLinkProps {
  id: number
  name: string
}

export default function PokemonLink({
  id,
  name,
}: PokemonLinkProps): JSX.Element {
  return (
    <Link href={`/pokemons/${name}`} className={styles.link} id={id.toString()}>
      {name}
      <PokemonImage id={id} size="sm" />
    </Link>
  )
}
