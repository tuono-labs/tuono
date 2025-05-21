use tuono_lib::axum::http::Request;
use tuono_lib::Response;
use tuono_lib_macros::handler;
async fn handler(request: Request) -> Response {
    Response::new(200)
}
pub async fn tuono_internal_route(
    request: Request,
) -> impl tuono_lib::axum::response::IntoResponse {
    handler(request.clone()).await.render_to_string(request)
}
pub async fn tuono_internal_api(
    request: Request,
) -> impl tuono_lib::axum::response::IntoResponse {
    handler(request.clone()).await.json()
}
