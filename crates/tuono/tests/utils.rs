use fs_extra::dir::create_all;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use wiremock::matchers::query_param;
use wiremock::matchers::QueryParamExactMatcher;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

#[derive(Debug)]
pub struct TempTuonoProject {
    original_dir: PathBuf,
    temp_dir: TempDir,
}

impl TempTuonoProject {
    pub fn new() -> Self {
        let project = TempTuonoProject::new_with_no_config();

        project.add_file("./tuono.config.ts");

        project
    }

    pub fn new_with_no_config() -> Self {
        let original_dir = env::current_dir().expect("Failed to read current_dir");
        let temp_dir = tempdir().expect("Failed to create temp_dir");

        env::set_current_dir(temp_dir.path()).expect("Failed to change current dir into temp_dir");

        TempTuonoProject {
            original_dir,
            temp_dir,
        }
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn add_file(&self, path: &str) -> File {
        let path = PathBuf::from(path);
        create_all(
            path.parent().expect("Route path does not have any parent"),
            false,
        )
        .expect("Failed to create parent route directory");
        File::create(path).expect("Failed to create the route file")
    }

    pub fn add_file_with_content<'a>(&self, path: &'a str, content: &'a str) {
        let path = PathBuf::from(path);
        create_all(
            path.parent().expect("Route path does not have any parent"),
            false,
        )
        .expect("Failed to create parent route directory");

        let mut file = File::create(path).expect("Failed to create the route file");
        file.write_all(content.as_bytes())
            .expect("Failed to write into API file");
    }
}

impl Drop for TempTuonoProject {
    fn drop(&mut self) {
        // Set back the current dir in the previous state
        env::set_current_dir(&self.original_dir)
            .expect("Failed to restore the original directory.");
    }
}

pub struct MockServerWrapper {
    server: MockServer,
}

pub enum ResponseBody {
    Json(Value),
    String(String),
}

impl MockServerWrapper {
    pub async fn new() -> Self {
        MockServerWrapper {
            server: MockServer::start().await,
        }
    }

    pub async fn register_mock(
        &self,
        method: &str,
        path: &str,
        params: Option<QueryParamExactMatcher>,
        status: u16,
        response_body: ResponseBody,
    ) {
        env::set_var("GITHUB_API_BASE_UR", self.server.uri());
        env::set_var("GITHUB_RAW_CONTENT_BASE_URL", self.server.uri());

        let mut mock = Mock::given(matchers::method(method)).and(matchers::path(path));

        if let Some(params) = params {
            mock = mock.and(params);
        }

        let response_template = match response_body {
            ResponseBody::Json(body) => ResponseTemplate::new(status).set_body_json(body),
            ResponseBody::String(body) => ResponseTemplate::new(status).set_body_string(body),
        };

        mock.respond_with(response_template)
            .mount(&self.server)
            .await;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mockserverwrapper_register_mock_with_json_response() {
        let mock_server = MockServerWrapper::new().await;

        let response_body = ResponseBody::Json(json!({"key": "value"}));
        mock_server
            .register_mock("GET", "/test", None, 200, response_body)
            .await;

        let response = reqwest::get(&format!("{}/test", mock_server.server.uri()))
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body: Value = response.json().await.expect("Failed to parse JSON");
        assert_eq!(body, json!({"key": "value"}));
    }

    #[tokio::test]
    async fn test_mockserverwrapper_register_mock_with_string_response() {
        let mock_server = MockServerWrapper::new().await;

        let response_body = ResponseBody::String("Hello, world!".to_string());
        mock_server
            .register_mock("GET", "/hello", None, 200, response_body)
            .await;

        let response = reqwest::get(&format!("{}/hello", mock_server.server.uri()))
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn test_mockserverwrapper_register_mock_with_query_params() {
        let mock_server = MockServerWrapper::new().await;

        let response_body = ResponseBody::String("Query matched".to_string());
        let query_matcher = query_param("key", "value");

        mock_server
            .register_mock("GET", "/query", Some(query_matcher), 200, response_body)
            .await;

        let response = reqwest::get(&format!("{}/query?key=value", mock_server.server.uri()))
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);

        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(body, "Query matched");
    }
}
