use tuono_lib::Response;
use tuono_lib_macros::handler;

#[handler]
async fn handler() -> Response {
    Response::new(200)
}
