use std::env;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

use tuono_lib::EnvVarManager;
use tuono_lib::Mode;

#[test]
fn integration_test_env_var_manager() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let original_dir = env::current_dir().expect("failed to get current dir");

    env::set_current_dir(&temp_dir).expect("failed to change directory");

    {
        let mut file = File::create(".env").expect("failed to create .env");
        writeln!(file, "FOO=bar").expect("failed to write to .env");
    }

    {
        let mut file = File::create(".env.local").expect("failed to create .env.local");
        writeln!(file, "IGNORED=should_not_override").expect("failed to write to .env.local");
    }

    {
        let mut file = File::create(".env.development").expect("failed to create .env.development");
        writeln!(file, "BAZ=qux").expect("failed to write to .env.development");
    }

    {
        let mut file = File::create(".env.development.local")
            .expect("failed to create .env.development.local");
        writeln!(file, "HELLO=world").expect("failed to write to .env.development.local");
    }

    env::set_var("IGNORED", "system_value");

    let manager = EnvVarManager::new(Mode::Dev);

    assert_eq!(manager.env_vars.get("FOO"), Some(&"bar".to_string()));
    assert_eq!(manager.env_vars.get("BAZ"), Some(&"qux".to_string()));
    assert_eq!(manager.env_vars.get("HELLO"), Some(&"world".to_string()));

    assert_eq!(
        manager.env_vars.get("IGNORED"),
        Some(&"system_value".to_string())
    );

    manager.load_into_env();
    assert_eq!(env::var("FOO").unwrap(), "bar");
    assert_eq!(env::var("BAZ").unwrap(), "qux");
    assert_eq!(env::var("HELLO").unwrap(), "world");
    assert_eq!(env::var("IGNORED").unwrap(), "system_value");

    env::set_current_dir(original_dir).expect("failed to restore current directory");
}
