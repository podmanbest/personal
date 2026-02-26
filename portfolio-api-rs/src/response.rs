//! Format respons API mengikuti OpenAPI 3: ApiResponse & ApiError.
//! Selaras dengan portfolio-api (Lumen): `data`, `message`, `errors`.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Respons sukses (OpenAPI ApiResponse).
/// Bentuk: `{ "data": ..., "message": "...", "errors": null }`.
/// Bentuk respons sukses: data, message, errors (null).
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: message.into(),
            errors: None,
        }
    }
}

/// Respons error (OpenAPI ApiError).
/// Bentuk: `{ "data": null, "message": "...", "errors": ... }`.
/// Bentuk respons error: data null, message, errors (opsional).
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(message: impl Into<String>, errors: Option<serde_json::Value>) -> Self {
        Self {
            data: None,
            message: message.into(),
            errors,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// Helper: respons JSON sukses dengan status code.
pub fn json_success<T: Serialize>(
    data: T,
    message: impl Into<String>,
    status: StatusCode,
) -> Response {
    let body = ApiResponse::<T>::success(data, message);
    (status, Json(body)).into_response()
}

/// Helper: respons error JSON dengan status code.
pub fn json_error(
    message: impl Into<String>,
    errors: Option<serde_json::Value>,
    status: StatusCode,
) -> Response {
    let body = ApiError::new(message, errors);
    (status, Json(body)).into_response()
}

// #region agent log
#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[test]
    fn api_response_success_creates_correct_struct() {
        let r = ApiResponse::<i32>::success(42, "OK");
        assert_eq!(r.data, 42);
        assert_eq!(r.message, "OK");
        assert!(r.errors.is_none());
    }

    #[test]
    fn api_response_serializes_with_data_and_message() {
        let r = ApiResponse::<serde_json::Value>::success(
            serde_json::json!({"id": 1}),
            "Done",
        );
        let json = serde_json::to_value(&r).unwrap();
        assert_eq!(json.get("data").unwrap().get("id").unwrap(), 1);
        assert_eq!(json.get("message").unwrap(), "Done");
        assert!(json.get("errors").is_none());
    }

    #[test]
    fn api_error_new_creates_correct_struct() {
        let e = ApiError::new("Not found", None);
        assert_eq!(e.message, "Not found");
        assert!(e.data.is_none());
        assert!(e.errors.is_none());
    }

    #[test]
    fn api_error_serializes_with_validation_errors() {
        let errs = serde_json::json!({"email": ["Invalid email"]});
        let e = ApiError::new("Validation failed", Some(errs.clone()));
        let json = serde_json::to_value(&e).unwrap();
        assert_eq!(json.get("message").unwrap(), "Validation failed");
        assert_eq!(json.get("errors").unwrap().get("email").unwrap().get(0).unwrap(), "Invalid email");
    }

    #[tokio::test]
    async fn json_success_returns_correct_status_and_body() {
        let res = json_success(vec!["a", "b"], "List OK", StatusCode::OK);
        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "List OK");
        assert!(json["data"].is_array());
        assert_eq!(json["data"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn json_success_created_returns_201() {
        let res = json_success(serde_json::json!({}), "Created", StatusCode::CREATED);
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn json_error_returns_correct_status_and_body() {
        let res = json_error("User not found", None, StatusCode::NOT_FOUND);
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "User not found");
    }

    #[tokio::test]
    async fn json_error_422_includes_errors_field() {
        let errs = serde_json::json!({"name": ["Required"]});
        let res = json_error("Validation failed", Some(errs), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["errors"]["name"][0], "Required");
    }
}
// #endregion
