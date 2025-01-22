mod utils;
use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use utils::TempTuonoProject;

const POST_API_FILE: &str = r"#[tuono_lib::api(POST)]";
const GET_API_FILE: &str = r"#[tuono_lib::api(GET)]";

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

    let temp_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.tuono/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/index.rs"]"#));
    assert!(temp_main_rs_content.contains("mod index;"));

    assert!(temp_main_rs_content
        .contains(r#".route("/", get(index::tuono__internal__route)).route("/__tuono/data/", get(index::tuono__internal__api))"#));
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

    let temp_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.tuono/main.rs' content.");

    dbg!(&temp_main_rs_content);

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_health_check;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post__tuono_internal_api))"#
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

    let temp_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.tuono/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_health_check;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post__tuono_internal_api))"#
    ));
    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", get(api_health_check::get__tuono_internal_api))"#
    ));
}

#[test]
#[serial]
fn it_successfully_create_catch_all_routes() {
    let temp_tuono_project = TempTuonoProject::new();

    temp_tuono_project.add_file("./src/routes/[...all_routes].rs");

    temp_tuono_project.add_file_with_content(
        "./src/routes/api/[...all_apis].rs",
        &format!("{POST_API_FILE}"),
    );

    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_tuono_project.path().join(".tuono/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.tuono/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/[...all_apis].rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_dyn_catch_all_all_apis;"));

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/[...all_routes].rs"]"#));
    assert!(temp_main_rs_content.contains("mod dyn_catch_all_all_routes;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/*all_apis", post(api_dyn_catch_all_all_apis::post__tuono_internal_api))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/*all_routes", get(dyn_catch_all_all_routes::tuono__internal__route))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/*all_routes", get(dyn_catch_all_all_routes::tuono__internal__route))"#
    ));

    assert!(temp_main_rs_content
        .contains(r#".route("/__tuono/data/*all_routes", get(dyn_catch_all_all_routes::tuono__internal__api))"#));
}

#[test]
#[serial]
fn it_fails_without_installed_build_config_script() {
    TempTuonoProject::new();

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

    temp_tuono_project
        .add_file_with_content("./node_modules/.bin/tuono-build-config", "#!/bin/bash");
    Command::new("chmod")
        .arg("+x")
        .arg("./node_modules/.bin/tuono-build-config")
        .assert()
        .success();
    let mut test_tuono_build = Command::cargo_bin("tuono").unwrap();
    test_tuono_build
        .arg("build")
        .assert()
        .failure()
        .stderr("[CLI] Failed to read tuono.config.ts\n");
}
