use crate::mode::Mode;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Clone, Debug)]
pub struct EnvVarManager {
    env_files: Vec<String>,
    system_env_names: HashSet<String>,
    pub env_vars: HashMap<String, String>,
}

impl EnvVarManager {
    pub fn new(mode: Mode) -> Self {
        let mut env_files = vec![String::from(".env"), String::from(".env.local")];

        let mode_name = match mode {
            Mode::Dev => "development",
            Mode::Prod => "production",
        };

        env_files.push(format!(".env.{}", mode_name));
        env_files.push(String::from(".env.local"));
        env_files.push(format!(".env.{}.local", mode_name));

        let system_env_names: HashSet<String> = env::vars().map(|(k, _)| k).collect();
        let env_vars: HashMap<String, String> = env::vars().collect();

        let mut manager = Self {
            env_files,
            system_env_names,
            env_vars,
        };

        manager.reload_variables();
        manager.load_into_env();
        manager
    }

    fn reload_variables(&mut self) {
        for env_file in &self.env_files {
            if let Ok(contents) = fs::read_to_string(env_file) {
                for line in contents.lines() {
                    if let Some((key, mut value)) = line.split_once('=') {
                        if value.starts_with('"') && value.ends_with('"') {
                            value = &value[1..value.len() - 1];
                        }

                        let key = key.trim().to_string();
                        let value = value.trim().to_string();

                        if self.system_env_names.contains(&key) {
                            continue; // Skip if key exists in system env
                        }

                        self.env_vars.insert(key, value);
                    }
                }
            }
        }

        self.load_into_env();
    }

    fn load_into_env(&self) {
        for (key, value) in &self.env_vars {
            env::set_var(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::Mode;
    use serial_test::serial;

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
            env::set_var(k, v);
        }

        pub fn setup_env_file(&mut self, file_name: &str, contents: &str) {
            self.files.push(file_name.to_string());
            fs::write(file_name, contents).expect("Failed to write test .env file");
        }
    }

    impl Drop for MockEnv {
        fn drop(&mut self) {
            for file in self.files.iter() {
                _ = fs::remove_file(file.as_str());
            }

            for var in self.vars.iter() {
                env::remove_var(var.0)
            }
        }
    }

    #[test]
    #[serial]
    fn test_system_env_var_precedence() {
        let mut env = MockEnv::new();

        env.add_system_var("TEST_KEY", "system_value");

        env.setup_env_file(".env", "TEST_KEY=file_value");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("TEST_KEY").unwrap(), "system_value");
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "TEST_KEY=base_value");
        env.setup_env_file(".env.development", "TEST_KEY=development_value");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("TEST_KEY").unwrap(), "development_value");
    }

    #[test]
    #[serial]
    fn test_local_env_var_precedence() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "TEST_KEY=base_value");
        env.setup_env_file(".env.local", "TEST_KEY=local_value");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_value");
    }

    #[test]
    #[serial]
    fn test_mode_local_env_var_precedence() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "TEST_KEY=base_value");
        env.setup_env_file(".env.development", "TEST_KEY=development_value");
        env.setup_env_file(".env.development.local", "TEST_KEY=local_dev_value");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_dev_value");
    }

    #[test]
    #[serial]
    fn test_empty_env_file() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert!(env::var("NON_EXISTENT_KEY").is_err());
    }

    #[test]
    #[serial]
    fn test_malformed_env_entries() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "INVALID_LINE\nMISSING_EQUALS_SIGN");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert!(env::var("INVALID_LINE").is_err());
        assert!(env::var("MISSING_EQUALS_SIGN").is_err());
    }

    #[test]
    #[serial]
    fn test_quoted_values_parsing() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", r#"TEST_KEY="quoted_value""#);

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("TEST_KEY").unwrap(), "quoted_value");
    }

    #[test]
    #[serial]
    fn test_non_existent_env_file() {
        let mut env = MockEnv::new();

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert!(env::var("NON_EXISTENT_KEY").is_err());
    }

    #[test]
    #[serial]
    fn test_multiple_env_vars() {
        let mut env = MockEnv::new();

        env.setup_env_file(".env", "KEY1=value1\nKEY2=value2");

        let manager = EnvVarManager::new(Mode::Dev);

        manager.load_into_env();

        for env_var in manager.env_vars {
            env.vars.insert(env_var.0, env_var.1);
        }

        assert_eq!(env::var("KEY1").unwrap(), "value1");
        assert_eq!(env::var("KEY2").unwrap(), "value2");
    }
}
