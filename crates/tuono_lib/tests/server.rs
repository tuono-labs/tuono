use serial_test::serial;
mod utils;
use crate::utils::utils::MockTuonoServer;
use std::sync::Once;

static mut SERVER: Option<MockTuonoServer> = None;
static INIT: Once = Once::new();

fn get_server() -> &'static MockTuonoServer {
    unsafe {
        INIT.call_once(|| {
            SERVER = Some(futures::executor::block_on(MockTuonoServer::spawn()));
        });

        SERVER.as_ref().unwrap()
    }
}

#[tokio::test]
#[serial]
async fn health_check_works() {
    println!("Running health_check_works");
    let app = get_server();
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
async fn return_not_found_html() {
    println!("Running return_not_found_html");
    let app = get_server();

    let client = reqwest::Client::builder().build().unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/not-found"))
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: This should return a 404
    assert!(response.status().is_success());
    assert_eq!(
        response.text().await.unwrap(),
        "<h1>404 Not found</h1><a href=\"/\">Return home</a>"
    );
}

#[tokio::test]
#[serial]
async fn return_route_html() {
    println!("Running return_route_html");
    let app = get_server();

    let client = reqwest::Client::builder().build().unwrap();

    let server_url = format!("http://{}:{}", &app.address, &app.port);

    let response = client
        .get(&format!("{server_url}/"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert!(response
        .text()
        .await
        .unwrap()
        .starts_with("<!DOCTYPE html>"));
}
