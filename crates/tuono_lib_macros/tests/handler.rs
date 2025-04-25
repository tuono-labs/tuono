use tuono_lib::axum::extract::Query;
use tuono_lib::cookie::CookieJar;
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
    #[tuono_lib_macros::handler(axum_arguments((_cookies)))]
    async fn get_foo(_req: Request, bar: String, _cookies: CookieJar) -> Response {
        Response::Props(Props::new(bar))
    }
}

#[test]
fn handler_with_axum_arguments_no_state() {
    // If this compiles, it's a success
    #[allow(dead_code)]
    #[tuono_lib_macros::handler(axum_arguments(
        (s, extractor = Query),
        (_cookies)
    ))]
    async fn get_foo(_req: Request, _cookies: CookieJar, s: String) -> Response {
        Response::Props(Props::new(s))
    }
}

#[test]
fn handler_with_axum_arguments_reverse_order() {
    // If this compiles, it's a success
    #[allow(dead_code)]
    #[tuono_lib_macros::handler(axum_arguments((_cookies)))]
    async fn get_foo(_req: Request, _cookies: CookieJar, bar: String) -> Response {
        Response::Props(Props::new(bar))
    }
}
