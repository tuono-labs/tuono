use assert_cmd::Command;
use clap::crate_version;
use serial_test::serial;

use std::fs;

mod utils;

use utils::mock_github_endpoint::GitHubServerMock;
use utils::temp_tuono_project::TempTuonoProject;

#[tokio::test]
#[serial]
async fn it_creates_a_new_project_and_replace_the_versions_in_the_manifest() {
    let GitHubServerMock { env_vars, .. } = GitHubServerMock::new().await;

    let temp_project = TempTuonoProject::new();

    let project_folder = "my-project";

    let mut cmd = Command::cargo_bin("tuono").unwrap();

    cmd.arg("new")
        .arg(project_folder)
        .envs(env_vars)
        .assert()
        .success();

    let main_rs_path = temp_project
        .path()
        .join(project_folder)
        .join("src")
        .join("main.rs");

    let cargo_toml_path = temp_project.path().join(project_folder).join("Cargo.toml");
    let package_json_path = temp_project
        .path()
        .join(project_folder)
        .join("package.json");

    let main_rs_content = fs::read_to_string(main_rs_path).unwrap();
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();
    let package_json_content = fs::read_to_string(package_json_path).unwrap();

    assert_eq!(
        main_rs_content,
        "fn main() { println!(\"Hello, world!\"); }"
    );

    let version = crate_version!();

    let expected_cargo_toml_content =
        format!("[package] name = \"tuono-tutorial\" [dependencies] tuono_lib = \"{version}\"");

    assert_eq!(cargo_toml_content, expected_cargo_toml_content);

    let expected_package_json_content =
        format!("{{\"name\": \"tuono-app\", \"dependencies\": {{ \"tuono\": \"{version}\" }}}}");

    assert_eq!(package_json_content, expected_package_json_content);
}
