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
