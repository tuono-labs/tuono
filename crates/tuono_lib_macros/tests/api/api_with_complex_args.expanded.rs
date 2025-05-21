use serde::{Deserialize, Serialize};
use tuono_lib::axum::extract::{Json, Query, State};
use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;
struct AppState;
struct AppQuery {
    page: usize,
    per_page: usize,
}
async fn api(
    State(state): State<AppState>,
    query: Query<AppQuery>,
    Json(body): Json<serde_json::Value>,
) -> StatusCode {
    StatusCode::OK
}
pub async fn post_tuono_internal_api(
    arg_0: State<AppState>,
    query: Query<AppQuery>,
    arg_2: Json<serde_json::Value>,
) -> StatusCode {
    api(arg_0, query, arg_2).await
}
