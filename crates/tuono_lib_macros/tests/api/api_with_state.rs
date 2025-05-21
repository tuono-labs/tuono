use tuono_lib::axum::extract::State;
use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;

struct AppState;

#[api(GET)]
async fn api(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}
