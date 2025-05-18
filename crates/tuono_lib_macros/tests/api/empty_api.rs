use tuono_lib::axum::http::StatusCode;
use tuono_lib_macros::api;

#[api(GET)]
async fn api() -> StatusCode {
    StatusCode::OK
}
