use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};
mod utils;
use assert_cmd::Command;
use serial_test::serial;
use utils::TempTuonoProject;

#[derive(Deserialize, Debug, Serialize)]
pub enum GithubFileType {
    #[serde(rename = "blob")]
    Blob,
    #[serde(rename = "tree")]
    Tree,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GithubTagObject {
    sha: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct GithubTagResponse {
    object: GithubTagObject,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GithubTreeResponse<T> {
    tree: Vec<T>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GithubFile {
    path: String,
    #[serde(rename = "type")]
    element_type: GithubFileType,
}

#[tokio::test]
#[serial]
async fn it_successfully_mocks_github_api_call() {
    // Arrange
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;
    // Set the base URI for the mock server
    let base_uri = mock_server.uri();
    std::env::set_var("ENVIRONMENT", "test");
    std::env::set_var("BASE_URI", mock_server.uri());

    // Get Sha
    Mock::given(method("GET"))
        .and(path("v0.17.3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(GithubTagResponse {
            object: GithubTagObject {
                sha: "mysha".to_string(),
            },
        }))
        .mount(&mock_server)
        .await;

        // Get Tree 
        Mock::given(method("GET"))
            .and(path("/mysha"))
            .and(wiremock::matchers::query_param("recursive", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(GithubTreeResponse::<GithubFile> {
                tree: vec![
                    GithubFile {
                        path: "Hello".to_string(),
                        element_type: GithubFileType::Blob
                    },
                    GithubFile {
                        path: "dfdfdfd".to_string(),
                        element_type: GithubFileType::Tree
                    }
                ]
            }))
            .mount(&mock_server)
            .await;

      
    let temp_tuono_project = TempTuonoProject::new();

    let mut test_tuono_new = Command::cargo_bin("tuono").unwrap();
    test_tuono_new
        .arg("new")
        .arg("my_new_project")
        .assert()
        .success();

    // let new_project_path = temp_tuono_project.path().join("my_new_project");
    // test_tuono_new
    //     .arg("new")
    //     .arg(temp_tuono_project.path())
    //     .assert()
    //     .success();

    // assert!(new_project_path.exists());
    // assert!(new_project_path.join("Cargo.toml").exists());
    // assert!(new_project_path.join("src/main.rs").exists());
}
