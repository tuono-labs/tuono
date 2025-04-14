// src/routes/index.rs
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tuono_lib::{Props, Request, Response};

const ALL_POKEMON: &str = "https://pokeapi.co/api/v2/pokemon?limit=151";

#[derive(Debug, Serialize, Deserialize)]
struct Pokemons {
    results: Vec<Pokemon>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    url: String,
}

#[tuono_lib::handler]
async fn get_all_pokemons(_req: Request, fetch: Client) -> Response {
    match fetch.get(ALL_POKEMON).send().await {
        Ok(res) => {
            let data = res.json::<Pokemons>().await.unwrap();
            Response::Props(Props::new(data))
        }
        Err(_err) => Response::Props(Props::new_with_status(
            "{}", // Return empty JSON
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
