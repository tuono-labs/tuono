// File automatically generated
// Do not manually change it

use tuono_lib::{axum::Router, tuono_internal_init_v8_platform, Mode, Server};
use std::future::Future;

//AXUM_GET_ROUTE_HANDLER//

const MODE: Mode = //MODE//;

//MODULE_IMPORTS//

//APP_STATE_IMPORT//

pub async fn get_router<F, Fut>(f: F) -> Router
where
    //GET_ROUTER_SIGNATURE//
    Fut: Future<Output = Router>,
{
    //APP_STATE_DECLARATION//
    let router = Router::new()
    //ROUTE_BUILDER//;
    //GET_ROUTER_USAGE//
}

pub async fn init_server_with_router<F, Fut>(f: F) -> Server
where
    //GET_ROUTER_SIGNATURE//
    Fut: Future<Output = Router>,
{
    tuono_internal_init_v8_platform();
    let router = get_router(f).await;
    if MODE == Mode::Prod {
        println!("\n  ⚡ Tuono v//VERSION//");
    }
    Server::init(router, MODE).await
}



pub async fn init_server() -> Server {
    init_server_with_router(
        //APP_STATE_USAGE//
    ).await
}
