// File automatically generated
// Do not manually change it

use tuono_lib::{tokio, Mode, Server, axum::Router, tuono_internal_init_v8_platform};
// AXUM_GET_ROUTE_HANDLER

const MODE: Mode = /*MODE*/;

// MODULE_IMPORTS

//APP_STATE_IMPORT//
//SERVER_IMPORT//

#[tokio::main]
async fn main() {
    tuono_internal_init_v8_platform();
    
    if MODE == Mode::Prod {
        println!("\n  ⚡ Tuono v/*VERSION*/");
    }

    //APP_STATE_DEFINITION//

    let router = Router::new()
        // ROUTE_BUILDER
        //APP_STATE_USAGE//;

    //SERVER_DEFINITION//

    Server::init(router, MODE, serve).await.start().await
}

