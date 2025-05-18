use serde::{Deserialize, Serialize};
use tuono_lib::axum::extract::{Json, Query, State};
use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;

struct AppState;

#[derive(Serialize, Deserialize)]
struct AppQuery {
    page: usize,
    per_page: usize,
}

#[api(POST)]
async fn api(
    State(state): State<AppState>,
    query: Query<AppQuery>,
    Json(body): Json<serde_json::Value>,
) -> StatusCode {
    StatusCode::OK
}
