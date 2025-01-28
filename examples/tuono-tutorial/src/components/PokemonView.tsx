import type { JSX } from 'react'

import { Pokemon } from '../models/Pokemon'

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
          <dt className={styles.label}>Weight: </dt>
          <dd>{pokemon.weight}lbs</dd>
        </dl>
        <dl className={styles.spec}>
          <dt className={styles.label}>Height: </dt>
          <dd>{pokemon.height}ft</dd>
        </dl>
      </div>
      <img
        src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/${pokemon.id}.png`}
        alt=""
      />
    </div>
  )
}
