use crate::config::GLOBAL_CONFIG;
use crate::manifest::load_manifest;
use crate::mode::{GLOBAL_MODE, Mode};
use axum::routing::{Router, get};
use colored::Colorize;
use ssr_rs::Ssr;
use tower_http::services::ServeDir;
use tuono_internal::config::Config;
use tuono_internal::tuono_println;

use crate::env::load_env_vars;
use crate::{
    catch_all::catch_all, services::logger::LoggerLayer, vite_reverse_proxy::vite_reverse_proxy,
    vite_websocket_proxy::vite_websocket_proxy,
};

const DEV_PUBLIC_DIR: &str = "public";
const PROD_PUBLIC_DIR: &str = "out/client";

pub fn tuono_internal_init_v8_platform() {
    Ssr::create_platform();
}

#[derive(Debug)]
pub struct Server {
    router: Router,
    mode: Mode,
    pub listener: tokio::net::TcpListener,
    pub address: String,
    pub origin: Option<String>,
}

impl Server {
    fn display_start_message(&self) {
        // Format the server address as a valid URL so that it becomes clickable in the CLI
        // @see https://github.com/tuono-labs/tuono/issues/460
        let server_base_url = format!("http://{}", self.address);

        // In order to avoid multiple logs on `tuono dev`
        // the server address prompt for tuono dev is made on the CLI process
        if self.mode == Mode::Prod {
            tuono_println!("Production server at: {}\n", server_base_url.blue().bold());
            if let Some(origin) = &self.origin {
                tuono_println!("Origin: {}\n", origin.blue().bold());
            }
        } else {
            tuono_println!("Ready\n");
        }
    }

    pub async fn init(router: Router, mode: Mode) -> Server {
        let config = Config::get().expect("[SERVER] Failed to load config");

        let _ = GLOBAL_MODE.set(mode);
        let _ = GLOBAL_CONFIG.set(config.clone());

        if mode == Mode::Prod {
            if let Err(err) = load_manifest() {
                tuono_println!("Failed to load vite manifest: {}", err.to_string().red());
            }
        }

        let server_address = format!("{}:{}", config.server.host, config.server.port);

        unsafe {
            // This function is unsafe because it modifies the OS env variables
            // which is not thread-safe.
            // However, we are using it in a controlled environment which hasn't
            // spawned any threads yet.
            load_env_vars(mode);
        }

        Server {
            router,
            mode,
            address: server_address.clone(),
            origin: config.server.origin.clone(),
            listener: tokio::net::TcpListener::bind(&server_address)
                .await
                .expect("[SERVER] Failed to bind to address"),
        }
    }

    pub async fn start(self) {
        self.display_start_message();

        if self.mode == Mode::Dev {
            let router = self
                .router
                .to_owned()
                .layer(LoggerLayer::new())
                .route("/vite-server/", get(vite_websocket_proxy))
                .route("/vite-server/{*path}", get(vite_reverse_proxy))
                .fallback_service(
                    ServeDir::new(DEV_PUBLIC_DIR)
                        .fallback(get(catch_all).layer(LoggerLayer::new())),
                );

            axum::serve(self.listener, router)
                .await
                .expect("Failed to serve development server");
        } else {
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
