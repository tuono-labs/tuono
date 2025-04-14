// src/routes/pokemons/[pokemon].rs
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tuono_lib::{Props, Request, Response};

const POKEMON_API: &str = "https://pokeapi.co/api/v2/pokemon";

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    id: u16,
    weight: u16,
    height: u16,
}

#[tuono_lib::handler]
async fn get_pokemon(req: Request, fetch: Client) -> Response {
    // The param `pokemon` is defined in the route filename [pokemon].rs
    let pokemon = req.params.get("pokemon").unwrap();

    match fetch.get(format!("{POKEMON_API}/{pokemon}")).send().await {
        Ok(res) => {
            if res.status() == StatusCode::NOT_FOUND {
                return Response::Props(Props::new_with_status("{}", StatusCode::NOT_FOUND));
            }
            let data = res.json::<Pokemon>().await.unwrap();
            Response::Props(Props::new(data))
        }
        Err(_err) => Response::Props(Props::new_with_status(
            "{}",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
