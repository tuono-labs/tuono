use tuono_lib::{Props, Request, Response};

#[tuono_lib::handler]
async fn route(_: Request) -> Response {
    Response::Props(Props::new("{}"))
}
