use tuono_lib::axum::extract::Path;
use tuono_lib::cookie::CookieJar;
use tuono_lib::Response;
use tuono_lib_macros::handler;
async fn handler(
    cookies: CookieJar,
    request: tuono_lib::axum::http::Request,
    Path((one, two, three)): Path<(String, String, String)>,
) -> Response {
    Response::new(200)
}
pub async fn tuono_internal_route(
    cookies: CookieJar,
    request: tuono_lib::axum::http::Request,
    arg_2: Path<(String, String, String)>,
) -> impl tuono_lib::axum::response::IntoResponse {
    handler(cookies, request.clone(), arg_2).await.render_to_string(request)
}
pub async fn tuono_internal_api(
    cookies: CookieJar,
    request: tuono_lib::axum::http::Request,
    arg_2: Path<(String, String, String)>,
) -> impl tuono_lib::axum::response::IntoResponse {
    handler(cookies, request.clone(), arg_2).await.json()
}
