use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExternalUrl {
    pub github_tuono_tags_url: String,
    pub github_tuono_tag_commit_trees_url: String,
    pub github_raw_content_url: String,
}

pub fn get_url() -> ExternalUrl {
    let mut github_tuono_tags_url =
        "https://api.github.com/repos/tuono-labs/tuono/git/ref/tags/".to_string();
    let mut github_tuono_tag_commit_trees_url =
        "https://api.github.com/repos/tuono-labs/tuono/git/trees/".to_string();
    let mut github_raw_content_url =
        "https://raw.githubusercontent.com/tuono-labs/tuono/".to_string();

    if env::var("ENVIRONMENT").unwrap() == "test" {
        let mock_url = env::var("MOCK_URI").unwrap() + "/";
        
        github_tuono_tags_url = mock_url.clone();
        github_tuono_tag_commit_trees_url = mock_url.clone();
        github_raw_content_url = mock_url.clone();
    }

    ExternalUrl {
        github_tuono_tags_url,
        github_tuono_tag_commit_trees_url,
        github_raw_content_url,
    }
}
