use tuono_lib::axum::extract::Path;
use tuono_lib::cookie::CookieJar;
use tuono_lib::Response;
use tuono_lib_macros::handler;

#[handler]
async fn handler(
    cookies: CookieJar,
    request: tuono_lib::axum::http::Request,
    Path((one, two, three)): Path<(String, String, String)>,
) -> Response {
    Response::new(200)
}
