mod utils;
use serial_test::serial;
use std::io::ErrorKind;
use tuono_internal::config::Config;
use utils::TempTuonoProject;

#[test]
#[serial]
fn should_correctly_read_the_config_file() {
    let folder = TempTuonoProject::new();

    folder.add_file_with_content(
        "./.tuono/config/config.json",
        r#"{ "server": {"host": "localhost", "port": 3000}}"#,
    );

    let config = Config::get();

    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.origin, None);
    assert_eq!(config.server.port, 3000);
}

#[test]
#[serial]
fn should_correctly_read_the_config_file_with_origin() {
    let folder = TempTuonoProject::new();

    folder.add_file_with_content(
        "./.tuono/config/config.json",
        r#"{ "server": {"host": "localhost", "origin": "https://tuono.localhost", "port": 3000}}"#,
    );

    let config = Config::get();

    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.server.host, "localhost".to_string());
    assert_eq!(
        config.server.origin,
        Some("https://tuono.localhost".to_string())
    );
    assert_eq!(config.server.port, 3000);
}

#[test]
#[serial]
fn should_fail_if_the_file_does_not_exist() {
    TempTuonoProject::new();

    let config = Config::get();

    assert!(config.is_err());

    assert_eq!(config.err().unwrap().kind(), ErrorKind::NotFound);
}

#[test]
#[serial]
fn should_fail_if_the_file_is_not_json() {
    let folder = TempTuonoProject::new();

    folder.add_file_with_content("./.tuono/config/config.json", "INVALID JSON");

    let config = Config::get();

    assert!(config.is_err());

    assert_eq!(config.err().unwrap().kind(), ErrorKind::InvalidData);
}
