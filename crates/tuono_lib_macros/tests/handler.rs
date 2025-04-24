use tuono_lib::axum::http::StatusCode;
use tuono_lib::cookie::{Cookie, CookieJar};
use tuono_lib::{Props, Request, Response};

pub mod tuono_main_state {
    pub struct ApplicationState {
        pub bar: String,
    }
}

#[test]
fn handler_with_axum_arguments_and_state() {
    // If this compiles, it's a success
    #[allow(dead_code)]
    #[tuono_lib_macros::handler(axum_arguments(cookie_jar))]
    async fn get_foo(_req: Request, bar: String, cookie_jar: CookieJar) -> Response {
        let cookie = cookie_jar
            .get("baz")
            .cloned()
            .unwrap_or(Cookie::new("baz", "qux"));
        Response::Props(Props::new_with_status(
            vec![
                ("foo".to_string(), bar),
                ("baz".to_string(), cookie.to_string()),
            ],
            StatusCode::OK,
        ))
    }
}

#[test]
fn handler_with_axum_arguments_no_state() {
    // If this compiles, it's a success
    #[allow(dead_code)]
    #[tuono_lib_macros::handler(axum_arguments(cookie_jar))]
    async fn get_foo(_req: Request, cookie_jar: CookieJar) -> Response {
        let cookie = cookie_jar
            .get("baz")
            .cloned()
            .unwrap_or(Cookie::new("baz", "qux"));
        Response::Props(Props::new_with_status(
            vec![("baz".to_string(), cookie.to_string())],
            StatusCode::OK,
        ))
    }
}

#[test]
fn handler_with_axum_arguments_reverse_order() {
    // If this compiles, it's a success
    #[allow(dead_code)]
    #[tuono_lib_macros::handler(axum_arguments(cookie_jar))]
    async fn get_foo(_req: Request, cookie_jar: CookieJar, bar: String) -> Response {
        let cookie = cookie_jar
            .get("baz")
            .cloned()
            .unwrap_or(Cookie::new("baz", "qux"));
        Response::Props(Props::new_with_status(
            vec![
                ("foo".to_string(), bar),
                ("baz".to_string(), cookie.to_string()),
            ],
            StatusCode::OK,
        ))
    }
}
