use crate::mode::Mode;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
pub struct EnvVarManager {
    env_files: Vec<String>,
    env_names: HashSet<String>,
}

impl EnvVarManager {
    pub fn new(mode: Mode) -> Self {
        let mut env_files = vec![String::from(".env"), String::from(".env.local")];

        let mode_name = match mode {
            Mode::Dev => "development",
            Mode::Prod => "production",
        };

        env_files.push(format!(".env.{}", mode_name));
        env_files.push(format!(".env.{}.local", mode_name));

        let system_vars: HashSet<String> = env::vars().map(|(k, _)| k).collect();

        Self {
            env_files,
            env_names: system_vars,
        }
    }

    pub fn reload_variables(&self) {
        for env_file in self.env_files.iter() {
            if let Ok(contents) = fs::read_to_string(env_file) {
                for line in contents.lines() {
                    if let Some((key, mut value)) = line.split_once('=') {
                        if value.starts_with('"') && value.ends_with('"') {
                            value = &value[1..value.len() - 1];
                        }

                        let key = key.trim();
                        let value = value.trim();

                        if self.env_names.contains(key) {
                            // If the key exists in the system environment, skip setting it.
                            continue;
                        }

                        env::remove_var(key); // Ensure old .env values don't persist
                        env::set_var(key, value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::Mode;
    use serial_test::serial;

    fn setup_env_file(file_name: &str, contents: &str) {
        fs::write(file_name, contents).expect("Failed to write test .env file");
    }

    fn cleanup_env_files() {
        let _ = fs::remove_file(".env");
        let _ = fs::remove_file(".env.local");
        let _ = fs::remove_file(".env.development");
        let _ = fs::remove_file(".env.development.local");
        let _ = fs::remove_file(".env.production");
        let _ = fs::remove_file(".env.production.local");
    }

    #[test]
    #[serial]
    fn test_system_env_var_precedence() {
        env::set_var("TEST_KEY", "system_value");

        setup_env_file(".env", "TEST_KEY=file_value");
        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("TEST_KEY").unwrap(), "system_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.development", "TEST_KEY=development_value");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("TEST_KEY").unwrap(), "development_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_local_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.local", "TEST_KEY=local_value");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_mode_local_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.development", "TEST_KEY=development_value");
        setup_env_file(".env.development.local", "TEST_KEY=local_dev_value");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_dev_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_empty_env_file() {
        setup_env_file(".env", "");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert!(env::var("NON_EXISTENT_KEY").is_err());

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_malformed_env_entries() {
        setup_env_file(".env", "INVALID_LINE\nMISSING_EQUALS_SIGN");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert!(env::var("INVALID_LINE").is_err());
        assert!(env::var("MISSING_EQUALS_SIGN").is_err());

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_quoted_values_parsing() {
        setup_env_file(".env", r#"TEST_KEY="quoted_value""#);

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("TEST_KEY").unwrap(), "quoted_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_non_existent_env_file() {
        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert!(env::var("NON_EXISTENT_KEY").is_err());

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_multiple_env_vars() {
        setup_env_file(".env", "KEY1=value1\nKEY2=value2");

        let manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(env::var("KEY1").unwrap(), "value1");
        assert_eq!(env::var("KEY2").unwrap(), "value2");

        env::remove_var("KEY1");
        env::remove_var("KEY2");
        cleanup_env_files();
    }
}
