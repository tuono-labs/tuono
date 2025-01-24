use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct ExternalUrl {
    pub github_tuono_tags_url: String,
    pub github_tuono_tag_commit_trees_url: String,
    pub github_raw_content_url: String,
}

impl ExternalUrl {
    pub fn new() -> Self {
        if env::var("ENVIRONMENT").unwrap_or_default() == "test" {
            let mock_url = env::var("MOCK_URI").unwrap() + "/";
            return ExternalUrl {
                github_tuono_tags_url: mock_url.clone(),
                github_tuono_tag_commit_trees_url: mock_url.clone(),
                github_raw_content_url: mock_url.clone(),
            }
        }
        let base_uri = "https://api.github.com/repos/tuono-labs/tuono/";
        

        ExternalUrl {
            github_tuono_tags_url: format!("{base_uri}git/ref/tags/"),
            github_tuono_tag_commit_trees_url: format!("{base_uri}git/trees/"),
            github_raw_content_url: "https://raw.githubusercontent.com/tuono-labs/tuono/"
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_url_new_default() {
        env::remove_var("ENVIRONMENT");
        let external_url = ExternalUrl::new();
        assert_eq!(external_url.github_tuono_tags_url, "https://api.github.com/repos/tuono-labs/tuono/git/ref/tags/");
        assert_eq!(external_url.github_tuono_tag_commit_trees_url, "https://api.github.com/repos/tuono-labs/tuono/git/trees/");
        assert_eq!(external_url.github_raw_content_url, "https://raw.githubusercontent.com/tuono-labs/tuono/");
    }

    #[test]
    fn test_external_url_new_test_environment() {
        env::set_var("ENVIRONMENT", "test");
        env::set_var("MOCK_URI", "http://mockserver");
        let external_url = ExternalUrl::new();
        assert_eq!(external_url.github_tuono_tags_url, "http://mockserver/");
        assert_eq!(external_url.github_tuono_tag_commit_trees_url, "http://mockserver/");
        assert_eq!(external_url.github_raw_content_url, "http://mockserver/");
        env::remove_var("ENVIRONMENT");
        env::remove_var("MOCK_URI");
    }
}