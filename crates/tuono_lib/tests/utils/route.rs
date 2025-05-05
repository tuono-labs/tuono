use tuono_lib::{Props, Response};

#[tuono_lib::handler]
async fn route() -> Response {
    Response::Props(Props::new("{}"))
}
