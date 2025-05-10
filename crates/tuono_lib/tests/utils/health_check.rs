use tuono_lib::axum::http::StatusCode;

#[tuono_lib::api(GET)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}
