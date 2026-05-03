use axum::{BoxError, error_handling::HandleErrorLayer, http::StatusCode,};

#[tuono_lib::middleware]
pub fn api_error_layer() -> HandleErrorLayer  {
  HandleErrorLayer::new(|_: BoxError| async {
    StatusCode::REQUEST_TIMEOUT
})
}
