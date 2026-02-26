//! Integration test: endpoint status dan struktur respons (selaras AUDIT_API).
//! Menggunakan AppState::without_db() agar tidak butuh database.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use portfolio_api_rs::{app, AppState};
use tower::ServiceExt;

fn test_app() -> axum::Router {
    app(AppState::without_db())
}

#[tokio::test]
async fn get_root_returns_200_and_version() {
    let app = test_app();
    let req = Request::get("/").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("version").is_some());
    assert_eq!(json.get("name").and_then(|v| v.as_str()), Some("portfolio-api-rs"));
}

#[tokio::test]
async fn get_api_users_returns_200_and_data_message() {
    let app = test_app();
    let req = Request::get("/api/users").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("data").is_some());
    assert_eq!(json.get("message").and_then(|v| v.as_str()), Some("Users retrieved successfully"));
}

#[tokio::test]
async fn get_api_users_id_returns_200_for_valid_id() {
    let app = test_app();
    let req = Request::get("/api/users/1").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("data").is_some());
    assert_eq!(json.get("data").and_then(|d| d.get("id")).and_then(|v| v.as_u64()), Some(1));
}

#[tokio::test]
async fn get_api_users_0_returns_404() {
    let app = test_app();
    let req = Request::get("/api/users/0").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.get("message").and_then(|v| v.as_str()), Some("User not found"));
}

#[tokio::test]
async fn post_api_contact_returns_201() {
    let app = test_app();
    let body = br#"{"name":"Test","email":"a@b.com","subject":"S","message":"M"}"#;
    let req = Request::post("/api/contact")
        .header("content-type", "application/json")
        .body(Body::from(body.as_slice()))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let res_body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&res_body).unwrap();
    assert!(json.get("data").is_some());
    assert_eq!(json.get("message").and_then(|v| v.as_str()), Some("Message sent successfully."));
}

#[tokio::test]
async fn get_docs_returns_html() {
    let app = test_app();
    let req = Request::get("/docs").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok());
    assert!(ct.map(|s| s.contains("text/html")).unwrap_or(false));
}
