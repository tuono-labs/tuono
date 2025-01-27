use serde::Serialize;
use tuono::{Props, Request, Response};

#[derive(Serialize)]
struct MyResponse<'a> {
    subtitle: &'a str,
}

#[tuono::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(MyResponse {
        subtitle: "The react / rust fullstack framework",
    }))
}
