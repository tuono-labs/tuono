// File automatically generated
// Do not manually change it

use tuono_lib::{Mode, Server, axum::Router, tokio, tuono_internal_init_v8_platform};
// AXUM_GET_ROUTE_HANDLER

/*MODE*/

// MODULE_IMPORTS

//MAIN_FILE_IMPORT//

#[tokio::main]
async fn main() {
    tuono_internal_init_v8_platform();

    if MODE == Mode::Prod {
        println!("\n  ⚡ Tuono v/*VERSION*/");
    }

    //MAIN_FILE_DEFINITION//

    // ROUTE_BUILDER
    //MAIN_FILE_USAGE//

    Server::init(router, MODE).await.start().await
}
