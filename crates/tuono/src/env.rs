use crate::mode::Mode;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
pub struct EnvVarManager {
    env_files: Vec<String>,
    mode: Option<Mode>,
    system_env: HashSet<String>,
}

impl EnvVarManager {
    pub fn new(mode: Option<Mode>) -> Self {
        let mut env_files = vec![String::from(".env"), String::from(".env.local")];

        if let Some(mode) = &mode {
            let mode_name = match mode {
                Mode::Dev => "development",
                Mode::Prod => "production",
            };

            env_files.push(format!(".env.{}", mode_name));
            env_files.push(format!(".env.{}.local", mode_name));
        }

        // Collect system environment variable keys into a HashSet for fast lookup.
        let system_vars: HashSet<String> = env::vars().map(|(k, _)| k).collect();

        Self {
            env_files,
            mode,
            system_env: system_vars,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = Some(mode);
    }

    pub fn refresh_env_files(&self) {
        for env_file in self.env_files.iter() {
            if let Ok(contents) = fs::read_to_string(env_file) {
                for line in contents.lines() {
                    if let Some((key, mut value)) = line.split_once('=') {
                        if value.starts_with('"') && value.ends_with('"') {
                            value = &value[1..value.len() - 1];
                        }

                        let key = key.trim();
                        let value = value.trim();

                        if self.system_env.contains(key) {
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
        let manager = EnvVarManager::new(None);
        manager.refresh_env_files();

        assert_eq!(env::var("TEST_KEY").unwrap(), "system_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_mode_specific_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.development", "TEST_KEY=development_value");

        let manager = EnvVarManager::new(Some(Mode::Dev));
        manager.refresh_env_files();

        assert_eq!(env::var("TEST_KEY").unwrap(), "development_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_local_env_var_precedence() {
        setup_env_file(".env", "TEST_KEY=base_value");
        setup_env_file(".env.local", "TEST_KEY=local_value");

        let manager = EnvVarManager::new(None);
        manager.refresh_env_files();

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

        let manager = EnvVarManager::new(Some(Mode::Dev));
        manager.refresh_env_files();

        assert_eq!(env::var("TEST_KEY").unwrap(), "local_dev_value");

        env::remove_var("TEST_KEY");
        cleanup_env_files();
    }

    #[test]
    #[serial]
    fn test_env_var_manager_initialization_no_mode() {
        let manager = EnvVarManager::new(None);
        assert!(manager.env_files.contains(&".env".to_string()));
        assert!(manager.env_files.contains(&".env.local".to_string()));
        assert_eq!(manager.mode, None);
    }

    #[test]
    #[serial]
    fn test_env_var_manager_initialization_with_mode() {
        let manager = EnvVarManager::new(Some(Mode::Dev));
        assert!(manager.env_files.contains(&".env".to_string()));
        assert!(manager.env_files.contains(&".env.local".to_string()));
        assert!(manager.env_files.contains(&".env.development".to_string()));
        assert!(manager
            .env_files
            .contains(&".env.development.local".to_string()));
        assert_eq!(manager.mode, Some(Mode::Dev));
    }

    #[test]
    #[serial]
    fn test_set_mode() {
        let mut manager = EnvVarManager::new(None);
        manager.set_mode(Mode::Prod);
        assert_eq!(manager.mode, Some(Mode::Prod));
    }
}
