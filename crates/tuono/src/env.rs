use crate::mode::Mode;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Clone, Debug)]
pub struct EnvVarManager {
    env_files: Vec<String>,
    system_env_names: HashSet<String>,
    env_vars: HashMap<String, String>,
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

        manager.reload_variables(); // Load only missing env variables
        manager
    }

    pub fn reload_variables(&mut self) {
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
    }

    pub fn get_env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
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
        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("TEST_KEY"), Some(&"system_value".to_string()));

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.development", "TEST_KEY=development_value");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("TEST_KEY"), Some(&"development_value".to_string()));
        
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_local_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.local", "TEST_KEY=local_value");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("TEST_KEY"), Some(&"local_value".to_string()));

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_mode_local_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.development", "TEST_KEY=development_value");
        setup_env_file(".env.development.local", "TEST_KEY=local_dev_value");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("TEST_KEY"), Some(&"local_dev_value".to_string()));

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_empty_env_file() {
        setup_env_file(".env", "");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("NON_EXISTENT_KEY"), None);

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_malformed_env_entries() {
        setup_env_file(".env", "INVALID_LINE\nMISSING_EQUALS_SIGN");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("INVALID_LINE"), None);
        assert_eq!(manager.get_env_vars().get("MISSING_EQUALS_SIGN"), None);

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_quoted_values_parsing() {
        setup_env_file(".env", r#"TEST_KEY="quoted_value""#);

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("TEST_KEY"), Some(&"quoted_value".to_string()));

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_non_existent_env_file() {
        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("NON_EXISTENT_KEY"), None);

        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_multiple_env_vars() {
        setup_env_file(".env", "KEY1=value1\nKEY2=value2");

        let mut manager = EnvVarManager::new(Mode::Dev);
        manager.reload_variables();

        assert_eq!(manager.get_env_vars().get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(manager.get_env_vars().get("KEY2"), Some(&"value2".to_string()));
        
        cleanup_env_files();
    }
}
