mod utils;
use std::collections::HashMap;

use crate::utils::mock_server::MockTuonoServer;
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
        .get(format!("{server_url}/health_check"))
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
        .get(format!("{server_url}/not-found"))
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
        .get(format!("{server_url}/"))
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: This should return a 404 status code
    assert!(response.status().is_success());
    assert!(
        response
            .text()
            .await
            .unwrap()
            .starts_with("<!DOCTYPE html>")
    );
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
        .get(format!("{server_url}/tuono/data"))
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

#[tokio::test]
#[serial]
async fn it_reads_the_catch_all_path_parameter() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(format!("{server_url}/catch_all/url_parameter"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "url_parameter");
}

#[tokio::test]
#[serial]
async fn it_reads_the_path_parameter() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(format!("{server_url}/dynamic/url_parameter"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "url_parameter");
}

#[tokio::test]
#[serial]
async fn it_reads_an_env_var() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(format!("{server_url}/env"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "foobar");
}

#[tokio::test]
#[serial]
async fn it_parses_the_http_body() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .post(format!("{server_url}/api/post"))
        .body(r#"{"data":"payload"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "payload");
}

#[tokio::test]
#[serial]
async fn it_parses_the_form_encoded_url() {
    let app = MockTuonoServer::spawn().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let mut form_params = HashMap::new();
    form_params.insert("data", "payload");

    let response = client
        .post(format!("{server_url}/api/form_data"))
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&form_params)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "payload");
}
