use tuono_lib::axum::http::Request;
use tuono_lib::Response;
use tuono_lib_macros::handler;

#[handler]
async fn handler(request: Request) -> Response {
    Response::new(200)
}
