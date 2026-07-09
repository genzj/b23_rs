use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CleanResult {
    pub sanitized_url: String,
    pub raw_url: String,
    pub stripped_params: HashMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ResolverError {
    #[error("Failed to make HTTP request: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Invalid URL format")]
    InvalidUrl,
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),
}

static ALLOWED_PARAMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("p");
    set.insert("page");
    set.insert("t");
    set.insert("itemsid");
    set.insert("tab");
    set.insert("topic_id");
    set.insert("vote_id");
    set
});

pub async fn resolve_and_clean(client: &Client, url: &str) -> Result<CleanResult, ResolverError> {
    let raw_url = resolve_url(client, url).await?;
    clean_url(&raw_url)
}

async fn resolve_url(client: &Client, url: &str) -> Result<String, ResolverError> {
    // Basic validation
    let parsed = Url::parse(url).map_err(|_| ResolverError::InvalidUrl)?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(ResolverError::InvalidUrl);
    }

    // reqwest automatically follows redirects by default.
    // The final URL is available on the response object.
    let response = client.get(url).send().await?;

    // Check if the request was successful
    response.error_for_status_ref()?;

    Ok(response.url().to_string())
}

fn clean_url(url: &str) -> Result<CleanResult, ResolverError> {
    let mut parsed_url = Url::parse(url)?;

    let mut stripped_params = HashMap::new();
    let mut kept_pairs = Vec::new();

    for (k, v) in parsed_url.query_pairs() {
        let k_lower = k.to_lowercase();
        if ALLOWED_PARAMS.contains(&k_lower.as_str()) {
            kept_pairs.push((k.into_owned(), v.into_owned()));
        } else {
            stripped_params.insert(k.into_owned(), v.into_owned());
        }
    }

    parsed_url.query_pairs_mut().clear();

    if !kept_pairs.is_empty() {
        for (k, v) in kept_pairs {
            parsed_url.query_pairs_mut().append_pair(&k, &v);
        }
    }

    if parsed_url.path().ends_with('/') {
        let new_path = parsed_url.path().trim_end_matches('/').to_string();
        parsed_url.set_path(&new_path);
    }

    let mut sanitized_url = parsed_url.to_string();
    if sanitized_url.ends_with('?') {
        sanitized_url.pop();
    }

    Ok(CleanResult {
        sanitized_url,
        raw_url: url.to_string(),
        stripped_params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_and_clean_b23() {
        let mock_server = crate::test_helpers::setup_mock_server().await;
        let client = Client::new();
        let url = format!("{}/rlUSCcz", mock_server.uri());

        let result = resolve_and_clean(&client, &url).await.unwrap();

        // As per the specification acceptance criteria:
        assert_eq!(
            result.sanitized_url,
            format!("{}/video/BV1BECcB3EG6?p=1", mock_server.uri())
        );
        assert!(
            result
                .raw_url
                .starts_with(&format!("{}/video/BV1BECcB3EG6", mock_server.uri()))
        );
        // Make sure we stripped some params
        assert!(!result.stripped_params.is_empty());
    }

    #[test]
    fn test_clean_url_logic() {
        let raw = "https://www.bilibili.com/video/BV1BECcB3EG6?p=1&spm_id_from=333.999.0.0&vd_source=12345";
        let result = clean_url(raw).unwrap();

        assert_eq!(
            result.sanitized_url,
            "https://www.bilibili.com/video/BV1BECcB3EG6?p=1"
        );
        assert_eq!(result.raw_url, raw);
        assert_eq!(
            result
                .stripped_params
                .get("spm_id_from")
                .map(|s| s.as_str()),
            Some("333.999.0.0")
        );
        assert_eq!(
            result.stripped_params.get("vd_source").map(|s| s.as_str()),
            Some("12345")
        );
    }
}
