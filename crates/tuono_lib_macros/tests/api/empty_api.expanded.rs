use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;
async fn api() -> StatusCode {
    StatusCode::OK
}
pub async fn get_tuono_internal_api() -> StatusCode {
    api().await
}
