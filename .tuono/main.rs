
// File automatically generated
// Do not manually change it

use tuono_lib::{tokio, Mode, Server, axum::Router};


const MODE: Mode = Mode::Dev;

// MODULE_IMPORTS



#[tokio::main]
async fn main() {
    println!("\n  ⚡ Tuono v0.17.0");

    

    let router = Router::new()
        // ROUTE_BUILDER
        ;

    Server::init(router, MODE).start().await
}
