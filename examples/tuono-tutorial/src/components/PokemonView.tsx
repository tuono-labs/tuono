import type { JSX } from 'react'

import { Pokemon } from '../models/Pokemon'

import { PokemonImage } from './PokemonImage'

import styles from './PokemonView.module.css'

export default function PokemonView({
  pokemon,
}: {
  pokemon: Pokemon
}): JSX.Element {
  return (
    <div className={styles.pokemon}>
      <div>
        <h1 className={styles.name}>{pokemon.name}</h1>
        <dl className={styles.spec}>
          <dt className={styles.label}>Weight:</dt>
          <dd>{pokemon.weight}lbs</dd>
        </dl>
        <dl className={styles.spec}>
          <dt className={styles.label}>Height:</dt>
          <dd>{pokemon.height}ft</dd>
        </dl>
      </div>
      <PokemonImage id={pokemon.id} size="md" />
    </div>
  )
}
