// File automatically generated
// Do not manually change it

use tuono_lib::{axum::Router, tuono_internal_init_v8_platform, Mode, Server};

//AXUM_GET_ROUTE_HANDLER//

const MODE: Mode = //MODE//;

//MODULE_IMPORTS//

//APP_STATE_IMPORT//

pub async fn init_server() -> Server {
    tuono_internal_init_v8_platform();
    //APP_STATE_DECLARATION//
    let router = Router::new()
    //ROUTE_BUILDER//
    //APP_STATE_USAGE//;
    if MODE == Mode::Prod {
        println!("\n  ⚡ Tuono v//VERSION//");
    }
    Server::init(router, MODE).await
}
