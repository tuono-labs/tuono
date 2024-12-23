import { createRoute } from 'tuono';
import { dynamic } from 'external-lib';
const IndexImport = dynamic(() => import('./../src/routes/index'));
const PokemonspokemonImport = dynamic(() => import('./../src/routes/pokemons/[pokemon]'));
