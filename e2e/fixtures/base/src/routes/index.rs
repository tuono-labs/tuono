use serde::Serialize;
use tuono_lib::{Props, Response, Type};

#[derive(Serialize, Type)]
struct MyResponse<'a> {
    subtitle: &'a str,
}

#[tuono_lib::handler]
async fn get_server_side_props() -> Response {
    Response::Props(Props::new(MyResponse {
        subtitle: "Subtitle received from the server",
    }))
}
