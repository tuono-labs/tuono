use reqwest::Client;
use serde::{Deserialize, Serialize};
use tuono_lib::{Props, Request, Response};

const NORRIS_API: &str = "https://api.chucknorris.io/jokes/random?category=";

#[derive(Debug, Serialize, Deserialize)]
struct Fact {
    categories: Vec<String>,
    created_at: String,
    icon_url: String,
    id: String,
    updated_at: String,
    url: String,
    value: String,
}

#[tuono_lib::handler]
async fn get_random_fact(req: Request, fetch: Client) -> Response {
    let category = req.params.get("name").unwrap();
    let url = format!("{}{}", NORRIS_API, category);
    return match fetch.get(url).send().await {
        Ok(res) => {
            let data = res.json::<Fact>().await.unwrap();
            Response::Props(Props::new(data))
        }
        Err(_err) => Response::Props(Props::new("{}")),
    };
}
