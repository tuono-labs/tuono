use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use tracing::Level;

mod utils;
use utils::temp_tuono_project::TempTuonoProject;

const POST_API_FILE: &str = r"#[tuono_lib::api(POST)]";
const GET_API_FILE: &str = r"#[tuono_lib::api(GET)]";
const CUSTOM_MAIN_FILE: &str = r"
async fn main() {
    // this is custom
}
";

fn tracing_message(level: Level, module: &str, message: &str) -> String {
    format!("\x1b[31m{level}\x1b[0m \x1b[2mtuono::{module}\x1b[0m\x1b[2m:\x1b[0m {message}\n")
}

#[cfg(target_os = "windows")]
const BUILD_TUONO_CONFIG: &str = ".\\node_modules\\.bin\\tuono-build-config.cmd";

#[cfg(not(target_os = "windows"))]
const BUILD_TUONO_CONFIG: &str = "./node_modules/.bin/tuono-build-config";

#[test]
#[serial]
fn it_successfully_create_the_index_route() {
    let temp_tuono_project = TempTuonoProject::new();

    temp_tuono_project.add_file("./src/routes/index.rs");

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");

    let temp_main_rs_content = fs::read_to_string(&temp_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/index.rs"]"#));
    assert!(temp_main_rs_content.contains("mod index;"));

    assert!(temp_main_rs_content
        .contains(r#".route("/", get(index::tuono_internal_route)).route("/__tuono/data/", get(index::tuono_internal_api))"#));
}

#[test]
#[serial]
fn it_successfully_create_an_api_route() {
    let temp_tuono_project = TempTuonoProject::new();

    temp_tuono_project.add_file_with_content("./src/routes/api/health_check.rs", POST_API_FILE);

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");

    let temp_router_rs_content = fs::read_to_string(&temp_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");

    assert!(temp_router_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_router_rs_content.contains("mod api_health_check;"));

    assert!(temp_router_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post_tuono_internal_api))"#
    ));
}

#[test]
#[serial]
fn it_successfully_create_multiple_api_for_the_same_file() {
    let temp_tuono_project = TempTuonoProject::new();

    temp_tuono_project.add_file_with_content(
        "./src/routes/api/health_check.rs",
        &format!("{POST_API_FILE}{GET_API_FILE}"),
    );

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");

    let temp_main_rs_content = fs::read_to_string(&temp_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_health_check;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post_tuono_internal_api))"#
    ));
    assert!(
        temp_main_rs_content.contains(
            r#".route("/api/health_check", get(api_health_check::get_tuono_internal_api))"#
        )
    );
}

#[test]
#[serial]
fn it_successfully_import_mixed_case_routes() {
    let temp_tuono_project = TempTuonoProject::new();

    for method in ["get", "post", "put", "delete", "patch"] {
        temp_tuono_project.add_file_with_content(
            &format!("./src/routes/api/{method}_lower.rs"),
            &format!(r"#[tuono_lib::api({method})]"),
        );
        temp_tuono_project.add_file_with_content(
            &format!("./src/routes/api/{method}_upper.rs"),
            &format!(r"#[tuono_lib::api({})]", method.to_uppercase()),
        );
    }

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.tuono/main.rs' content.");

    for method in ["get", "post", "put", "delete", "patch"] {
        let expected = format!(r#"use tuono_lib::axum::routing::{method};"#);
        let imports = temp_main_rs_content.match_indices(&expected);
        assert_eq!(imports.count(), 1);
    }
}

#[test]
#[serial]
fn it_successfully_create_catch_all_routes() {
    let temp_tuono_project = TempTuonoProject::new();

    temp_tuono_project.add_file("./src/routes/[...all_routes].rs");

    temp_tuono_project.add_file_with_content("./src/routes/api/[...all_apis].rs", POST_API_FILE);

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");

    let temp_main_rs_content = fs::read_to_string(&temp_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/[...all_apis].rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_dyn_catch_all_all_apis;"));

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/[...all_routes].rs"]"#));
    assert!(temp_main_rs_content.contains("mod dyn_catch_all_all_routes;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/{*all_apis}", post(api_dyn_catch_all_all_apis::post_tuono_internal_api))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/{*all_routes}", get(dyn_catch_all_all_routes::tuono_internal_route))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/{*all_routes}", get(dyn_catch_all_all_routes::tuono_internal_route))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/__tuono/data/{*all_routes}", get(dyn_catch_all_all_routes::tuono_internal_api))"#
    ));
}

#[test]
#[serial]
fn it_fails_without_installed_build_config_script() {
    let _guard = TempTuonoProject::new();

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .assert()
        .failure()
        .stderr("Failed to find the build script. Please run `npm install`\n");
}

#[test]
#[serial]
fn it_fails_without_installed_build_script() {
    let temp_tuono_project = TempTuonoProject::new();
    temp_tuono_project.add_file_with_content(BUILD_TUONO_CONFIG, "#!/bin/bash");
    Command::new("chmod")
        .arg("+x")
        .arg(BUILD_TUONO_CONFIG)
        .assert()
        .success();
    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();

    test_tuono_build
        .arg("build")
        .assert()
        .failure()
        .stderr("Failed to read config. Please run `npm install` to generate automatically.\n");
}

#[test]
#[serial]
fn dev_fails_with_no_config() {
    let _temp_tuono_project = TempTuonoProject::new_with_no_config();

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("dev")
        .assert()
        .failure()
        .stdout(tracing_message(
            Level::ERROR,
            "source_builder",
            "Cannot find tuono.config.ts - is this a tuono project?",
        ));
}

#[test]
#[serial]
fn build_fails_with_no_config() {
    let _temp_tuono_project = TempTuonoProject::new_with_no_config();

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("dev")
        .assert()
        .failure()
        .stdout(tracing_message(
            Level::ERROR,
            "source_builder",
            "Cannot find tuono.config.ts - is this a tuono project?",
        ));
}

#[test]
#[serial]
fn it_uses_custom_main_when_present() {
    let temp_tuono_project = TempTuonoProject::new();
    temp_tuono_project.add_file_with_content("./src/main.rs", CUSTOM_MAIN_FILE);

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_tuono_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");
    assert!(!temp_tuono_main_rs_path.exists());

    let temp_tuono_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");
    assert!(temp_tuono_router_rs_path.exists());
}

#[test]
#[serial]
fn it_uses_stateful_get_router_function_when_stateful() {
    let temp_tuono_project = TempTuonoProject::new();
    temp_tuono_project.add_file_with_content(
        "./src/app.rs",
        r#"
        pub fn main() -> ApplicationState;
        "#,
    );

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_tuono_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");
    let temp_tuono_router_rs_content = fs::read_to_string(&temp_tuono_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");
    assert!(
        temp_tuono_router_rs_content
            .contains(r#"F: Fn(Router<ApplicationState>, ApplicationState) -> Fut,"#)
    );
    assert!(
        temp_tuono_router_rs_content.contains(
            r#"|router, user_custom_state| async { router.with_state(user_custom_state) }"#
        )
    );
}

#[test]
#[serial]
fn it_uses_stateless_get_router_function_when_stateless() {
    let temp_tuono_project = TempTuonoProject::new();
    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_tuono_router_rs_path = temp_tuono_project.path().join(".tuono/router.rs");
    let temp_tuono_router_rs_content = fs::read_to_string(&temp_tuono_router_rs_path)
        .expect("Failed to read '.tuono/router.rs' content.");
    assert!(temp_tuono_router_rs_content.contains(r#"F: Fn(Router) -> Fut,"#));
    assert!(temp_tuono_router_rs_content.contains(r#"|router| async { router }"#));
}
