use assert_cmd::Command;
use clap::crate_version;
use serial_test::serial;
use std::fs;
use wiremock::matchers::query_param;
mod utils;

use utils::{MockServerWrapper, ResponseBody, TempTuonoProject};

#[tokio::test]
#[serial]
async fn test_scaffold_project() {
    let mock_server = MockServerWrapper::new().await;

    let cli_version: &str = crate_version!();

    let sha = "1234567890abcdef";
    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("repos/tuono-labs/tuono/git/ref/tags/v{}", cli_version),
        None,
        200,
        ResponseBody::Json(serde_json::json!({
            "object": {
                "sha": sha
            }
        })),
    )
    .await;

    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("repos/tuono-labs/tuono/git/trees/{}", sha),
        Some(query_param("recursive", "1")),
        200,
        ResponseBody::Json(serde_json::json!({
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
        })),
    )
    .await;

    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("tuono-labs/tuono/v{cli_version}/examples/tuono-app/src/main.rs"),
        None,
        200,
        ResponseBody::String(String::from("fn main() { println!(\"Hello, world!\"); }")),
    )
    .await;

    MockServerWrapper::register_mock(
        &mock_server,
    "GET",
    &format!("tuono-labs/tuono/v{cli_version}/examples/tuono-app/Cargo.toml"),
    None,
    200,
    ResponseBody::String(String::from("[package] name = \"tuono-tutorial\" version = \"0.0.1\" edition = \"2021\" [[bin]] name = \"tuono\" path = \".tuono/main.rs\" [dependencies] tuono_lib = { path = \"../../crates/tuono_lib/\"} serde = { version = \"1.0.202\", features = [\"derive\"] } reqwest = \"0.12.9\"")))
        .await;

    let temp_project = TempTuonoProject::new();

    let main_rs_path = temp_project
        .path()
        .join("my-project")
        .join("src")
        .join("main.rs");

    let cargo_toml_path = temp_project.path().join("my-project").join("Cargo.toml");

    let mut cmd = Command::cargo_bin("tuono").unwrap();

    cmd.arg("new")
        .arg(temp_project.path().join("my-project"))
        .assert()
        .success();

    assert!(main_rs_path.exists());
    assert!(cargo_toml_path.exists());

    let main_rs_content = fs::read_to_string(main_rs_path).unwrap();
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();

    assert_eq!(
        main_rs_content,
        "fn main() { println!(\"Hello, world!\"); }"
    );
    let expected_cargo_toml_content = format!(
        "[package] name = \"tuono-tutorial\" version = \"0.0.1\" edition = \"2021\" [[bin]] name = \"tuono\" path = \".tuono/main.rs\" [dependencies] tuono_lib = {} serde = {{ version = \"1.0.202\", features = [\"derive\"] }} reqwest = \"0.12.9\"",  "{ path = \"../../crates/tuono_lib/\"}"
    );

    assert_eq!(cargo_toml_content, expected_cargo_toml_content);
}

#[serial]
#[tokio::test]
async fn test_scaffold_project_with_invalid_version() {
    let mock_server = MockServerWrapper::new().await;

    let invalid_version = "invalid_version";
    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("tuono-labs/tuono/git/ref/tags/v{}", invalid_version),
        None,
        404,
        ResponseBody::Json(serde_json::json!({
            "message": "Not Found"
        })),
    )
    .await;

    let temp_project = TempTuonoProject::new();

    let mut cmd = Command::cargo_bin("tuono").unwrap();

    cmd.arg("new")
        .arg(temp_project.path().join("my-invalid-project"))
        .assert()
        .failure();
}

#[serial]
#[tokio::test]
async fn test_scaffold_project_with_missing_files() {
    let mock_server = MockServerWrapper::new().await;

    let cli_version: &str = crate_version!();
    let sha = "1234567890abcdef";
    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("tuono-labs/tuono/git/ref/tags/v{}", cli_version),
        None,
        200,
        ResponseBody::Json(serde_json::json!({
            "object": {
                "sha": sha
            }
        })),
    )
    .await;

    MockServerWrapper::register_mock(
        &mock_server,
        "GET",
        &format!("git/trees/{}", sha),
        Some(query_param("recursive", "1")),
        200,
        ResponseBody::Json(serde_json::json!({
            "tree": [
                {
                    "path":  "examples/tuono-app/src",
                    "type": "tree"
                }
            ]
        })),
    )
    .await;

    let temp_project = TempTuonoProject::new();

    let main_rs_path = temp_project
        .path()
        .join("my-missing-files-project")
        .join("src")
        .join("main.rs");

    let cargo_toml_path = temp_project
        .path()
        .join("my-missing-files-project")
        .join("Cargo.toml");

    let mut cmd = Command::cargo_bin("tuono").unwrap();

    cmd.arg("new")
        .arg(temp_project.path().join("my-missing-files-project"))
        .assert()
        .failure();

    assert!(!main_rs_path.exists());
    assert!(!cargo_toml_path.exists());
}
