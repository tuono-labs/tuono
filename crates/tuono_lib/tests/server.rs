mod utils;
use utils::MockTuonoServer;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = MockTuonoServer::spawn().await;
    let client = reqwest::Client::new();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
