use serde::Serialize;
use tuono_lib::{Props, Request, Response, Type};

#[derive(Serialize, Type)]
struct EnvVarsResponse {
    server_var: String,
    public_var: String,
}

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(EnvVarsResponse {
        server_var: std::env::var("SERVER_TEST_VAR").unwrap_or_default(),
        public_var: std::env::var("TUONO_PUBLIC_TEST_VAR").unwrap_or_default(),
    }))
}
