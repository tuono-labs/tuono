mod utils;
use crate::utils::utils::MockTuonoServer;
use serial_test::serial;

#[tokio::test]
#[serial]
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
}

#[tokio::test]
#[serial]
async fn not_found_route() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

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

#[tokio::test]
#[serial]
async fn index_html_route() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/"))
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: This should return a 404 status code
    assert!(response.status().is_success());
    assert!(response
        .text()
        .await
        .unwrap()
        .starts_with("<!DOCTYPE html>"));
}

#[tokio::test]
#[serial]
async fn api_route_route() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/tuono/data"))
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: This should return a 404 status code
    assert!(response.status().is_success());
    assert_eq!(
        response.text().await.unwrap(),
        "{\"data\":\"{}\",\"info\":{\"redirect_destination\":null}}"
    );
}
