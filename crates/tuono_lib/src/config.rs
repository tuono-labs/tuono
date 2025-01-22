use once_cell::sync::OnceCell;
use tuono_internal::config::Config;

pub static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();
