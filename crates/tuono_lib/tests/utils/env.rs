use std::env;

#[tuono_lib::api(GET)]
pub async fn test_env() -> String {
    env::var("MY_TEST_KEY").unwrap_or("error".parse().unwrap())
}
