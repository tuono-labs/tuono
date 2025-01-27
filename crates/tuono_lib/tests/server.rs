mod utils;
use crate::utils::utils::MockTuonoServer;

/// Why a single test for all the endpoints?
///
/// Rust tests run all on the same process. No more than a single V8 instance
/// can run on the same process.
///
/// TODO: Find a way to better manage the integration tests for tuono_lib.
#[tokio::test]
async fn api_endpoint_work() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let response = client
        .get(&format!("{server_url}/not-found"))
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: This should return a 404 status code
    assert!(response.status().is_success());
    assert_eq!(
        response.text().await.unwrap(),
        "<h1>404 Not found</h1><a href=\"/\">Return home</a>"
    );
}
