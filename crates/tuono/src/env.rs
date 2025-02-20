use std::fs;
use std::env;

pub fn load_env_file() {
    // TODO: Support other names for env file, decide on precedence
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key.trim(), value.trim());
            }
        }
    }
}
