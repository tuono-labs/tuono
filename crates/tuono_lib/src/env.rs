use crate::mode::Mode;
use std::collections::HashSet;
use std::env;
use std::fs;

/// Read the env variables from the .env files
/// and set them in the OS env
///
/// This function is unsafe because it modifies the OS env variables (which needs
/// to be done in a single-threaded context).
pub unsafe fn load_env_vars(mode: Mode) {
    let mut env_files = vec![String::from(".env"), String::from(".env.local")];

    let mode_name = match mode {
        Mode::Dev => "development",
        Mode::Prod => "production",
    };

    env_files.push(format!(".env.{}", mode_name));
    env_files.push(String::from(".env.local"));
    env_files.push(format!(".env.{}.local", mode_name));

    let system_env_names: HashSet<String> = env::vars().map(|(k, _)| k).collect();

    for env_file in env_files {
        if let Ok(contents) = fs::read_to_string(env_file) {
            for line in contents.lines() {
                if let Some((key, mut value)) = line.split_once('=') {
                    if value.starts_with('"') && value.ends_with('"') {
                        value = &value[1..value.len() - 1];
                    }

                    let key = key.trim().to_string();
                    let value = value.trim().to_string();

                    if system_env_names.contains(&key) {
                        continue; // Skip if key exists in system env
                    }

                    unsafe {
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
    use std::collections::HashMap;
    use std::env;
    use std::fs;

    struct MockEnv {
        files: Vec<String>,
        vars: HashMap<String, String>,
    }

    impl MockEnv {
        fn new() -> Self {
            Self {
                files: Vec::new(),
                vars: HashMap::new(),
            }
        }

        fn add_system_var(&mut self, k: &str, v: &str) {
            self.vars.insert(k.to_string(), v.to_string());
            unsafe {
                env::set_var(k, v);
            }
        }

        pub fn setup_env_file(&mut self, file_name: &str, contents: &str) {
            self.files.push(file_name.to_string());
            fs::write(file_name, contents).expect("Failed to write test .env file");
        }

        pub fn capture_keys(&mut self, keys: &[&str]) {
            for key in keys {
                if let Ok(val) = env::var(key) {
                    self.vars.insert(key.to_string(), val);
                }
            }
        }
    }

    impl Drop for MockEnv {
        fn drop(&mut self) {
            for file in self.files.iter() {
                let _ = fs::remove_file(file.as_str());
            }
            for key in self.vars.keys() {
                unsafe {
                    env::remove_var(key);
                }
            }
        }
    }

    #[test]
    #[serial]
    fn test_system_env_var_precedence() {
        let mut mock_env = MockEnv::new();

        mock_env.add_system_var("TEST_KEY", "system_value");
        mock_env.setup_env_file(".env", "TEST_KEY=file_value");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "system_value");
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence_dev() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "TEST_KEY=base_value");
        mock_env.setup_env_file(".env.development", "TEST_KEY=development_value");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "development_value");
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence_prod() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "TEST_KEY=base_value");
        mock_env.setup_env_file(".env.production", "TEST_KEY=production_value");

        unsafe {
            load_env_vars(Mode::Prod);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "production_value");
    }

    #[test]
    #[serial]
    fn test_local_env_var_precedence() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "TEST_KEY=base_value");
        mock_env.setup_env_file(".env.local", "TEST_KEY=local_value");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_value");
    }

    #[test]
    #[serial]
    fn test_mode_local_env_var_precedence_dev() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "TEST_KEY=base_value");
        mock_env.setup_env_file(".env.development", "TEST_KEY=development_value");
        mock_env.setup_env_file(".env.development.local", "TEST_KEY=local_dev_value");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_dev_value");
    }

    #[test]
    #[serial]
    fn test_mode_local_env_var_precedence_prod() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "TEST_KEY=base_value");
        mock_env.setup_env_file(".env.production", "TEST_KEY=production_value");
        mock_env.setup_env_file(".env.production.local", "TEST_KEY=local_prod_value");

        unsafe {
            load_env_vars(Mode::Prod);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_prod_value");
    }

    #[test]
    #[serial]
    fn test_ignores_files_from_other_mode() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env.development", "TEST_KEY=development_value");
        mock_env.setup_env_file(".env.production", "TEST_KEY=production_value");

        unsafe {
            load_env_vars(Mode::Prod);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "production_value");
    }

    #[test]
    #[serial]
    fn test_empty_env_file() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        assert!(env::var("NON_EXISTENT_KEY").is_err());
    }

    #[test]
    #[serial]
    fn test_malformed_env_entries() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "INVALID_LINE\nMISSING_EQUALS_SIGN");
        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["INVALID_LINE", "MISSING_EQUALS_SIGN"]);

        assert!(env::var("INVALID_LINE").is_err());
        assert!(env::var("MISSING_EQUALS_SIGN").is_err());
    }

    #[test]
    #[serial]
    fn test_quoted_values_parsing() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", r#"TEST_KEY="quoted_value""#);

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["TEST_KEY"]);

        assert_eq!(env::var("TEST_KEY").unwrap(), "quoted_value");
    }

    #[test]
    #[serial]
    fn test_non_existent_env_file() {
        let mut mock_env = MockEnv::new();
        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["NON_EXISTENT_KEY"]);

        assert!(env::var("NON_EXISTENT_KEY").is_err());
    }

    #[test]
    #[serial]
    fn test_multiple_env_vars() {
        let mut mock_env = MockEnv::new();

        mock_env.setup_env_file(".env", "KEY1=value1\nKEY2=value2");

        unsafe {
            load_env_vars(Mode::Dev);
        }

        mock_env.capture_keys(&["KEY1", "KEY2"]);

        assert_eq!(env::var("KEY1").unwrap(), "value1");
        assert_eq!(env::var("KEY2").unwrap(), "value2");
    }
}
