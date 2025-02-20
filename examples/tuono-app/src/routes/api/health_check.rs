use std::env;
use tuono_lib::Request;
use tuono_lib::axum::http::StatusCode;

#[tuono_lib::api(GET)]
pub async fn health_check(_req: Request) -> StatusCode {
    println!("{}", env::var("DATABASE_URL").expect("DATABASE_URL not set"));
    StatusCode::OK
}
