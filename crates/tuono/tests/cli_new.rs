use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};
mod utils;

use utils::{set_up_mock_server, TempTuonoProject};

#[tokio::test]
#[serial]
async fn test_scaffold_project() {
    let mock_server = set_up_mock_server().await;

    Mock::given(method("GET"))
        .and(path("v0.17.3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "object": {
                "sha": "1234567890abcdef"
            }
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("1234567890abcdef"))
        .and(query_param("recursive", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "tree": [
                {
                    "path":  "examples/tuono-app/src",
                    "type": "tree"
                },
                {
                    "path": "examples/tuono-app/src/main.rs",
                    "type": "blob"
                },
                {
                    "path": "examples/tuono-app/Cargo.toml",
                    "type": "blob"
                },
                {
                    "path": "examples/tuono-app/package.json",
                    "type": "blob"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("v0.17.3/examples/tuono-app/src/main.rs"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("fn main() { println!(\"Hello, world!\"); }"),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("v0.17.3/examples/tuono-app/Cargo.toml"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "[package] name = \"tuono-tutorial\" version = \"0.0.1\" edition = \"2021\" [[bin]] name = \"tuono\" path = \".tuono/main.rs\" [dependencies] tuono_lib = { path = \"../../crates/tuono_lib/\"} serde = { version = \"1.0.202\", features = [\"derive\"] } reqwest = \"0.12.9\""
        ))
        .mount(&mock_server)
        .await;

    let temp_project = TempTuonoProject::new();

    let main_rs_path = temp_project.path().join("new").join("src").join("main.rs");
    let cargo_toml_path = temp_project.path().join("new").join("Cargo.toml");

    let mut cmd = Command::cargo_bin("tuono").unwrap();
    cmd.arg("new")
        .arg(temp_project.path().join("new"))
        .assert()
        .success();

    assert!(main_rs_path.exists());
    assert!(cargo_toml_path.exists());

    let main_rs_content = fs::read_to_string(main_rs_path).unwrap();
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();

    println!("main rs contec {:?}", main_rs_content);

    assert_eq!(
        main_rs_content,
        "fn main() { println!(\"Hello, world!\"); }"
    );
    assert_eq!(cargo_toml_content, "[package] name = \"tuono-tutorial\" version = \"0.0.1\" edition = \"2021\" [[bin]] name = \"tuono\" path = \".tuono/main.rs\" [dependencies] tuono_lib = \"0.17.3\" serde = { version = \"1.0.202\", features = [\"derive\"] } reqwest = \"0.12.9\"");
}
