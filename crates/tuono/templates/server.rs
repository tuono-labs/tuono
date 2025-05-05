// File automatically generated
// Do not manually change it

mod router;

use tuono_lib::tokio;

#[tokio::main]
async fn main() {
    let server = router::init_server().await;
    server.start().await
}
