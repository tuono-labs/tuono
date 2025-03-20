use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use axum::http::{HeaderMap, Uri};
use std::fmt;

/// Location must match client side interface
#[derive(Serialize, Debug)]
pub struct Location {
    href: String,
    pathname: String,
    #[serde(rename(serialize = "searchStr"))]
    search_str: String,
    search: HashMap<String, String>,
}

impl Location {
    pub fn pathname(&self) -> &String {
        &self.pathname
    }
}

impl From<Uri> for Location {
    fn from(uri: Uri) -> Self {
        let query = uri.query().unwrap_or("");
        Location {
            // TODO: build correct href
            href: uri.to_string(),
            pathname: uri.path().to_string(),
            search_str: query.to_string(),
            search: serde_urlencoded::from_str(query).unwrap_or(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub uri: Uri,
    pub headers: HeaderMap,
    pub params: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

impl Request {
    pub fn new(uri: Uri, headers: HeaderMap, params: HashMap<String, String>) -> Request {
        Request {
            uri,
            headers,
            params,
            body: None
        }
    }


    pub fn body_as_string(&self) -> Result<String, String> {
        match &self.body {
            Some(body) => String::from_utf8(body.clone()).map_err(|_| "Invalid UTF-8 in body".into()),
            None => Err("Body is missing".into()),
        }
    }

    pub fn location(&self) -> Location {
        Location::from(self.uri.to_owned())
    }

    pub fn body<'de, T: Deserialize<'de>>(&'de self) -> Result<T, BodyParseError> {
        if let Some(body) = self.headers.get("body") {
            if let Ok(body) = body.to_str() {
                let body = serde_json::from_str::<T>(body)?;
                return Ok(body);
            }
            return Err(BodyParseError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read body",
            )));
        }
        Err(BodyParseError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No body found",
        )))
    }

    pub fn form_data<T>(&mut self) -> Result<T, FormError>
    where
        T: DeserializeOwned
    {
        let content_type = self.headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.contains("application/x-www-form-urlencoded") {
            return Err(FormError::InvalidContentType);
        }

        let body_str = match self.body_as_string() {
            Ok(s) => s,
            Err(e) => return Err(FormError::ParseError(e))
        };

        let form_data = match form_urlencoded::parse(body_str.as_bytes())
            .into_owned()
            .collect::<HashMap<String, String>>()
        {
            data => {
                match serde_json::to_value(data) {
                    Ok(value) => value,
                    Err(e) => return Err(FormError::ParseError(e.to_string())),
                }
            },
        };

        let result: T = match serde_json::from_value(form_data) {
            Ok(value) => value,
            Err(e) => return Err(FormError::ParseError(e.to_string())),
        };
        return Ok(result);
    }
}

#[derive(Debug)]
pub enum BodyParseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<serde_json::Error> for BodyParseError {
    fn from(err: serde_json::Error) -> BodyParseError {
        BodyParseError::Serde(err)
    }
}

#[derive(Debug)]
pub enum FormError {
    InvalidContentType,
    ParseError(String),
    ServerError(String),
}

impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormError::InvalidContentType => write!(f, "Invalid content type, expected form data"),
            FormError::ParseError(msg) => write!(f, "Failed to parse form data: {}", msg),
            FormError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl axum::response::IntoResponse for FormError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            FormError::InvalidContentType =>
                axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            FormError::ParseError(_) =>
                axum::http::StatusCode::BAD_REQUEST,
            FormError::ServerError(_) =>
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = axum::Json(serde_json::json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, Deserialize)]
    struct FakeBody {
        field1: bool,
        field2: String,
    }


    #[derive(Debug, Deserialize)]
    struct FormData {
        name: String,
        email: Option<String>,
    }

    #[test]
    fn it_correctly_parse_the_body() {
        let mut request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        request.headers.insert(
            "body",
            r#"{"field1": true, "field2": "hello"}"#.parse().unwrap(),
        );

        let body: FakeBody = request.body().expect("Failed to parse body");

        assert!(body.field1);
        assert_eq!(body.field2, "hello".to_string());
    }

    #[test]
    fn it_should_trigger_an_error_when_no_body_is_found() {
        let request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        let body: Result<FakeBody, BodyParseError> = request.body();

        assert!(body.is_err());
    }

    #[test]
    fn it_should_trigger_an_error_when_body_is_invalid() {
        let mut request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        request
            .headers
            .insert("body", r#"{"field1": true"#.parse().unwrap());

        let body: Result<FakeBody, BodyParseError> = request.body();

        assert!(body.is_err());
    }

    #[test]
    fn it_correctly_parses_form_data() {
        let mut request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        request.headers.insert(
            "content-type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        request.body = Some("name=John+Doe&email=john%40example.com".as_bytes().to_vec());

        let form_data: Result<FormData, FormError> = request.form_data();

        assert!(form_data.is_ok());
        let data = form_data.unwrap();
        assert_eq!(data.name, "John Doe");
        assert_eq!(data.email, Some("john@example.com".to_string()));
    }

    #[test]
    fn it_rejects_wrong_content_type() {
        let mut request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        request.headers.insert(
            "content-type",
            "application/json".parse().unwrap(),
        );

        request.headers.insert(
            "body",
            "name=John+Doe&email=john%40example.com".parse().unwrap(),
        );

        let form_data: Result<FormData, FormError> = request.form_data();

        assert!(form_data.is_err());
        match form_data.unwrap_err() {
            FormError::InvalidContentType => {},
            _ => panic!("Expected invalid content type error"),
        }
    }

    #[test]
    fn it_handles_missing_body() {
        let mut request = Request::new(
            Uri::from_static("http://localhost:3000"),
            HeaderMap::new(),
            HashMap::new(),
        );

        request.headers.insert(
            "content-type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );


        let form_data: Result<FormData, FormError> = request.form_data();

        assert!(form_data.is_err());
    }
}
