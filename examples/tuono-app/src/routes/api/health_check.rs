use tuono::Request;
use tuono::axum::http::StatusCode;

#[tuono::api(GET)]
pub async fn health_check(_req: Request) -> StatusCode {
    StatusCode::OK
}
