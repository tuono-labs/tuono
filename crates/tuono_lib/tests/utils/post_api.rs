use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    data: String,
}

#[tuono_lib::api(POST)]
async fn health_check(Json(body): Json<Payload>) -> String {
    body.data
}
