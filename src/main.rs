use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
    routing::get,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod resolver;

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
    let client = Client::new();
    let state = AppState { client };

    let app = Router::new()
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

    use http_body_util::BodyExt;
    use serde_json::Value;

    #[tokio::test]
    async fn test_clean_api_json() {
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/clean", get(clean_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/clean?url=https://b23.tv/rlUSCcz")
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
            "https://www.bilibili.com/video/BV1BECcB3EG6?p=1"
        );
    }

    #[tokio::test]
    async fn test_clean_api_text() {
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/clean", get(clean_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/clean?url=https://b23.tv/rlUSCcz&format=text")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(
            &body[..],
            b"https://www.bilibili.com/video/BV1BECcB3EG6?p=1"
        );
    }

    #[tokio::test]
    async fn test_redirect_api() {
        let state = AppState {
            client: Client::new(),
        };
        let app = Router::new()
            .route("/api/v1/redirect", get(redirect_handler))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/redirect?url=https://b23.tv/rlUSCcz")
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
            "https://www.bilibili.com/video/BV1BECcB3EG6?p=1"
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
