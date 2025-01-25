use fs_extra::dir::create_all;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::routing::get;
use tuono_lib::Request;
use tuono_lib::{axum::Router, Mode, Server};

#[derive(Debug)]
pub struct MockTuonoServer {
    pub address: String,
    pub port: u16,
    original_dir: PathBuf,
    #[allow(dead_code)]
    // Required for dropping the temp_dir when this struct drops
    temp_dir: TempDir,
}

fn add_file_with_content<'a>(path: &'a str, content: &'a str) {
    let path = PathBuf::from(path);
    create_all(
        path.parent().expect("File path does not have any parent"),
        false,
    )
    .expect("Failed to create parent file directories");

    let mut file = File::create(path).expect("Failed to create the file");
    file.write_all(content.as_bytes())
        .expect("Failed to write into the file");
}

impl MockTuonoServer {
    pub async fn spawn() -> Self {
        let original_dir = env::current_dir().expect("Failed to read current_dir");
        let temp_dir = tempdir().expect("Failed to create temp_dir");

        env::set_current_dir(temp_dir.path()).expect("Failed to change current dir into temp_dir");

        add_file_with_content(
            ".tuono/config/config.json",
            r#"{"server": {"host": "localhost", "port": 0}}"#,
        );

        add_file_with_content(
            "./out/client/.vite/manifest.json",
            r#"{"index.tsx": { "file": "assets/index.js", "name": "index", "src": "index.tsx", "isEntry": true,"dynamicImports": [],"css": []}}"#,
        );

        let router = Router::new().route("/health_check", get(get__tuono_internal_api));

        let server = Server::init(router, Mode::Prod).await;

        let socket = server
            .listener
            .local_addr()
            .expect("Failed to extract test server socket");

        let _ = tokio::spawn(server.start());

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
        env::set_current_dir(self.original_dir.to_owned())
            .expect("Failed to restore the original directory.");
    }
}

#[tuono_lib::api(GET)]
async fn index(_req: Request) -> StatusCode {
    StatusCode::OK
}
