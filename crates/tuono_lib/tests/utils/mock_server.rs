use fs_extra::dir::create_all;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};
use tempfile::{tempdir, TempDir};
use tuono_lib::axum::routing::get;
use tuono_lib::{axum::Router, tuono_internal_init_v8_platform, Mode, Server};

use crate::utils::catch_all::get_tuono_internal_api as catch_all;
use crate::utils::dynamic_parameter::get_tuono_internal_api as dynamic_parameter;
use crate::utils::health_check::get_tuono_internal_api as health_check;
use crate::utils::route as html_route;
use crate::utils::route::tuono_internal_api as route_api;
use crate::utils::env::get_tuono_internal_api as test_env;

use std::sync::Once;

static INIT: Once = Once::new();

fn init_v8() {
    INIT.call_once(|| {
        tuono_internal_init_v8_platform();
    })
}

fn add_file_with_content<'a>(path: &'a str, content: &'a str) {
    let path = PathBuf::from(path);
    create_all(
        path.parent().expect("File path does not have any parent"),
        false,
    )
    .unwrap_or_else(|_| {
        panic!(
            "Failed to create parent file directories: {}",
            path.display()
        )
    });

    let mut file = File::create(path).expect("Failed to create the file");
    file.write_all(content.as_bytes())
        .expect("Failed to write into the file");
}

#[derive(Debug)]
pub struct MockTuonoServer {
    pub address: String,
    pub port: u16,
    original_dir: PathBuf,
    #[allow(dead_code)]
    // Required for dropping the temp_dir when this struct drops
    temp_dir: TempDir,
}

impl MockTuonoServer {
    pub async fn spawn() -> Self {
        init_v8();
        let original_dir = env::current_dir().expect("Failed to read current_dir");
        let temp_dir = tempdir().expect("Failed to create temp_dir");

        let react_prod_build = fs::read_to_string("./tests/assets/fake_react_build.js")
            .expect("Failed to read fake_react_build.js");
        
        let env = fs::read_to_string("./tests/assets/.env")
            .expect("Failed to read .env");

        env::set_current_dir(temp_dir.path()).expect("Failed to change current dir into temp_dir");

        add_file_with_content(
            "./.tuono/config/config.json",
            r#"{"server": {"host": "127.0.0.1", "port": 0}}"#,
        );

        add_file_with_content("./out/server/prod-server.js", react_prod_build.as_str());

        add_file_with_content(
            "./out/client/.vite/manifest.json",
            r#"{"client-main.tsx": { "file": "assets/index.js", "name": "index", "src": "index.tsx", "isEntry": true,"dynamicImports": [],"css": []}}"#,
        );

        add_file_with_content("./.env", env.as_str());

        let router = Router::new()
            .route("/", get(html_route::tuono_internal_route))
            .route("/tuono/data", get(html_route::tuono_internal_api))
            .route("/health_check", get(health_check))
            .route("/route-api", get(route_api))
            .route("/catch_all/{*catch_all}", get(catch_all))
            .route("/dynamic/{parameter}", get(dynamic_parameter))
            .route("/env", get(test_env));

        let server = Server::init(router, Mode::Prod).await;

        let socket = server
            .listener
            .local_addr()
            .expect("Failed to extract test server socket");

        _ = tokio::spawn(server.start());

        MockTuonoServer {
            address: socket.ip().to_string(),
            port: socket.port(),
            original_dir,
            temp_dir,
        }
    }
}

impl Drop for MockTuonoServer {
    fn drop(&mut self) {
        // Set back the current dir in the previous state
        env::set_current_dir(&self.original_dir)
            .expect("Failed to restore the original directory.");
    }
}
