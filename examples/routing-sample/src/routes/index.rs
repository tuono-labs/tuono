use reqwest::Client;
use tuono_lib::{Props, Request, Response};

const NORRIS_API: &str = "https://api.chucknorris.io/jokes/categories";

#[tuono_lib::handler]
async fn get_random_fact(_req: Request, fetch: Client) -> Response {
    return match fetch.get(NORRIS_API).send().await {
        Ok(res) => {
            let data = res.json::<Vec<String>>().await.unwrap();
            Response::Props(Props::new(data))
        }
        Err(_err) => Response::Props(Props::new("{}")),
    };
}
