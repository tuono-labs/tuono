use tuono_lib::Response;
use tuono_lib_macros::handler;
async fn handler() -> Response {
    Response::new(200)
}
pub async fn tuono_internal_route(
    req: tuono_lib::axum::extract::Request,
) -> impl tuono_lib::axum::response::IntoResponse {
    handler().await.render_to_string(req)
}
pub async fn tuono_internal_api() -> impl tuono_lib::axum::response::IntoResponse {
    handler().await.json()
}
