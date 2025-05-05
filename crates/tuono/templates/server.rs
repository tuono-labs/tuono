// File automatically generated
// Do not manually change it

mod router;

use tuono_lib::{Mode, Server, axum::Router, tokio, tuono_internal_init_v8_platform};

#[tokio::main]
async fn main() {
    let server = router::init_server().await;
    server.start().await
}
