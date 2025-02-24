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

#[test]
#[serial]
fn it_inits_new_git_repo_by_default_with_git_installed() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new.arg("new").arg(".").assert().success();

    // Ensure the `.git` directory exists
    assert!(temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_does_not_init_new_git_repo_with_git_false() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .arg("--git-init=false")
        .assert()
        .success();

    // Ensure the `.git` directory does not exist
    assert!(!temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_creates_project_without_git_if_not_installed() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .env("PATH", "") // Simulate git not being installed
        .assert()
        .success();

    assert!(!temp_tuono_project.path().join(".git").exists());
}

#[test]
#[serial]
fn it_errors_if_git_not_installed_and_flag_set() {
    let temp_tuono_project = TempTuonoProject::new();

    std::env::set_current_dir(temp_tuono_project.path()).unwrap();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg(".")
        .arg("--git-init=true")
        .env("PATH", "") // Simulate git not being installed
        .assert()
        .failure()
        .stderr("You requested to use Git, but it is not installed.\n");
}
