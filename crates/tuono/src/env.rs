use std::fs;
use std::env;

pub fn load_server_env_file() {
    // TODO: Support other names for env file, decide on precedence
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, mut value)) = line.split_once('=') {
                // Remove surrounding double quotes from value if present
                if value.starts_with('"') && value.ends_with('"') {
                    value = &value[1..value.len() - 1];
                }

                env::set_var(key.trim(), value.trim());
            }
        }
    }
}

pub fn load_client_env_file() {
    // TODO: Support other names for env file, decide on precedence
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, mut value)) = line.split_once('=') {
                // Only allow public env vars to client
                if !key.trim().starts_with("TUONO_PUBLIC_") {
                    continue;
                }

                // Remove surrounding double quotes from value if present
                if value.starts_with('"') && value.ends_with('"') {
                    value = &value[1..value.len() - 1];
                }

                env::set_var(key.trim(), value.trim());
            }
        }
    }
}

