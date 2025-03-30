use serde::Deserialize;
use tuono_lib::Request;

#[derive(Deserialize)]
struct Payload {
    data: String,
}

#[tuono_lib::api(POST)]
async fn health_check(req: Request) -> String {
    req.body::<Payload>().unwrap().data
}
