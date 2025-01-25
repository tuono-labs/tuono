use crate::config::GLOBAL_CONFIG;
use crate::manifest::load_manifest;
use crate::mode::{Mode, GLOBAL_MODE};
use axum::routing::{get, Router};
use colored::Colorize;
use ssr_rs::Ssr;
use tower_http::services::ServeDir;
use tuono_internal::config::Config;

use crate::{
    catch_all::catch_all, logger::LoggerLayer, vite_reverse_proxy::vite_reverse_proxy,
    vite_websocket_proxy::vite_websocket_proxy,
};

const DEV_PUBLIC_DIR: &str = "public";
const PROD_PUBLIC_DIR: &str = "out/client";

#[derive(Debug)]
pub struct Server {
    router: Router,
    mode: Mode,
    pub listener: tokio::net::TcpListener,
    pub address: String,
}

impl Server {
    pub async fn init(router: Router, mode: Mode) -> Server {
        Ssr::create_platform();

        let config = Config::get().expect("[SERVER] Failed to load config");

        GLOBAL_MODE.set(mode).unwrap();
        GLOBAL_CONFIG.set(config.clone()).unwrap();

        if mode == Mode::Prod {
            load_manifest()
        }

        let server_http_address = format!("{}:{}", config.server.host, config.server.port);

        Server {
            router,
            mode,
            address: server_http_address.clone(),
            listener: tokio::net::TcpListener::bind(&server_http_address)
                .await
                .unwrap(),
        }
    }

    pub async fn start(self) {
        if self.mode == Mode::Dev {
            println!("  Ready at: {}\n", self.address.blue().bold());
            let router = self
                .router
                .to_owned()
                .layer(LoggerLayer::new())
                .route("/vite-server/", get(vite_websocket_proxy))
                .route("/vite-server/*path", get(vite_reverse_proxy))
                .fallback_service(
                    ServeDir::new(DEV_PUBLIC_DIR)
                        .fallback(get(catch_all).layer(LoggerLayer::new())),
                );

            axum::serve(self.listener, router)
                .await
                .expect("Failed to serve development server");
        } else {
            println!("  Production server at: {}\n", self.address.blue().bold());
            let router = self
                .router
                .to_owned()
                .layer(LoggerLayer::new())
                .fallback_service(
                    ServeDir::new(PROD_PUBLIC_DIR)
                        .fallback(get(catch_all).layer(LoggerLayer::new())),
                );

            axum::serve(self.listener, router)
                .await
                .expect("Failed to serve production server");
        }
    }
}
