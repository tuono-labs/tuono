use tuono_lib::axum::extract::State;
use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;
struct AppState;
async fn api(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}
pub async fn get_tuono_internal_api(arg_0: State<AppState>) -> StatusCode {
    api(arg_0).await
}
