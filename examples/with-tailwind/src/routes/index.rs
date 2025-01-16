use serde::Serialize;
use tuono_lib::{Props, Request, Response};

#[derive(Serialize)]
struct MyResponse<'a> {
    subtitle: &'a str,
}

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(MyResponse {
        subtitle: "The react / rust fullstack framework",
    }))
}
