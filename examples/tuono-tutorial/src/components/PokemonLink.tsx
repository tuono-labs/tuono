// src/components/PokemonLink.tsx
import type { JSX } from 'react'
import { Link } from 'tuono'
c
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
      <img
        src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${id}.png`}
        alt=""
      />
    </Link>
  )
}
