use std::fs;
use std::env;
use crate::mode::Mode;

pub fn load_env_files(mode: Option<Mode>) {
    let mut env_files = vec![String::from(".env"), String::from(".env.local")];

    if let Some(mode) = mode {
        let mode_name = match mode {
            Mode::Dev => "development",
            Mode::Prod => "production"
        };

        env_files.push(format!(".env.{}", mode_name));
        env_files.push(format!(".env.{}.local", mode_name));
    }

    for env_file in env_files {
        if let Ok(contents) = fs::read_to_string(env_file) {
            for line in contents.lines() {
                if let Some((key, mut value)) = line.split_once('=') {
                    if value.starts_with('"') && value.ends_with('"') {
                        value = &value[1..value.len() - 1];
                    }

                    let key = key.trim();
                    let value = value.trim();

                    if env::vars().any(|(k, _)| k == key) {
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
