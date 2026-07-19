use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::get,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod dns;
mod resolver;

#[cfg(test)]
pub mod test_helpers;
#[derive(Clone)]
struct AppState {
    client: Client,
}

#[derive(Deserialize)]
struct CleanQuery {
    url: String,
    #[serde(default)]
    format: Format,
}

#[derive(Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Format {
    #[default]
    Json,
    Text,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

const EN_JSON: &str = include_str!("locales/en.json");
const ZH_CN_JSON: &str = include_str!("locales/zh-cn.json");

async fn index_handler(headers: HeaderMap) -> Html<String> {
    let mut lang = "en";

    let cookie_str = headers
        .get(axum::http::header::COOKIE)
        .and_then(|c| c.to_str().ok())
        .unwrap_or("");

    if cookie_str.contains("lang=zh-cn") {
        lang = "zh-cn";
    } else if cookie_str.contains("lang=en") {
        lang = "en";
    } else {
        let accept_lang = headers
            .get(axum::http::header::ACCEPT_LANGUAGE)
            .and_then(|c| c.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        if accept_lang.contains("zh-cn") || accept_lang.starts_with("zh") {
            lang = "zh-cn";
        }
    }

    let i18n_json = match lang {
        "zh-cn" => ZH_CN_JSON,
        _ => EN_JSON,
    };

    let html = include_str!("index.html");
    let injected = format!(
        "<script>window.i18n = {}; window.currentLang = '{}';</script>\n</head>",
        i18n_json, lang
    );

    Html(html.replace("</head>", &injected))
}

// Handler for /api/v1/clean
async fn clean_handler(State(state): State<AppState>, Query(query): Query<CleanQuery>) -> Response {
    match resolver::resolve_and_clean(&state.client, &query.url).await {
        Ok(result) => {
            if query.format == Format::Text {
                result.sanitized_url.into_response()
            } else {
                Json(result).into_response()
            }
        }
        Err(e) => {
            let status = StatusCode::BAD_REQUEST;
            let body = Json(ErrorResponse {
                error: e.to_string(),
            });
            (status, body).into_response()
        }
    }
}

// Handler for /api/v1/redirect
async fn redirect_handler(
    State(state): State<AppState>,
    Query(query): Query<CleanQuery>,
) -> Response {
    match resolver::resolve_and_clean(&state.client, &query.url).await {
        Ok(result) => Redirect::temporary(&result.sanitized_url).into_response(),
        Err(e) => {
            let status = StatusCode::BAD_REQUEST;
            let body = Json(ErrorResponse {
                error: e.to_string(),
            });
            (status, body).into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    let domains_env =
        std::env::var("UPSTREAM_DOMAINS").unwrap_or_else(|_| "b23.tv,d.bilibili.com".to_string());
    let domains: Vec<String> = domains_env
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let secure_resolver = std::sync::Arc::new(crate::dns::SecureResolver::new(domains));
    let client = Client::builder()
        .dns_resolver(secure_resolver)
        .build()
        .expect("Failed to build HTTP client");

    let state = AppState { client };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/v1/clean", get(clean_handler))
        .route("/api/v1/redirect", get(redirect_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_health() {
        let app = Router::new().route("/api/health", get(|| async { "OK" }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_root_page_returns_embedded_html() {
        let app = Router::new().route("/", get(index_handler));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let page = std::str::from_utf8(&body).unwrap();
        assert!(page.contains("id=\"parameter-review\""));
        assert!(page.contains("stripped_params"));
    }

    use http_body_util::BodyExt;
    use serde_json::Value;

    #[tokio::test]
    async fn test_clean_api_json() {
        let mock_server = crate::test_helpers::setup_mock_server().await;
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/clean", get(clean_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/clean?url={}/rlUSCcz", mock_server.uri()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            json["sanitized_url"],
            format!("{}/video/BV1BECcB3EG6?p=1", mock_server.uri())
        );
    }

    #[tokio::test]
    async fn test_clean_api_text() {
        let mock_server = crate::test_helpers::setup_mock_server().await;
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/clean", get(clean_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/api/v1/clean?url={}/rlUSCcz&format=text",
                        mock_server.uri()
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(
            &body[..],
            format!("{}/video/BV1BECcB3EG6?p=1", mock_server.uri()).as_bytes()
        );
    }

    #[tokio::test]
    async fn test_redirect_api() {
        let mock_server = crate::test_helpers::setup_mock_server().await;
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/redirect", get(redirect_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/api/v1/redirect?url={}/rlUSCcz",
                        mock_server.uri()
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap(),
            format!("{}/video/BV1BECcB3EG6?p=1", mock_server.uri())
        );
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/clean", get(clean_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/clean?url=not-a-url")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
