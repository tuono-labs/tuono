// File automatically generated
// Do not manually change it

use tuono_lib::{tokio, Mode, Server, axum::Router, tuono_internal_init_v8_platform};
// AXUM_GET_ROUTE_HANDLER

const MODE: Mode = /*MODE*/;

// MODULE_IMPORTS

//MAIN_FILE_IMPORT//

#[tokio::main]
async fn main() {
    tuono_internal_init_v8_platform();
    println!("\n  ⚡ Tuono v/*VERSION*/");

    //MAIN_FILE_DEFINITION//

    let router = Router::new()
        // ROUTE_BUILDER
        //MAIN_FILE_USAGE//;

    Server::init(router, MODE).await.start().await
}
