//! Autentikasi Bearer token — selaras Lumen: Authorization Bearer <token>, user dari users.api_token.

use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};
use serde::Serialize;
use sqlx::FromRow;

use crate::response::json_error;
use crate::state::AppState;

/// User yang terautentikasi (tanpa password/api_token di response).
#[derive(Clone, Debug, Serialize, FromRow)]
pub struct CurrentUser {
    pub id: i64,
    pub full_name: String,
    pub username: Option<String>,
}

/// Resolve user dari Bearer token (header + pool). Dipanggil dari handler.
pub async fn resolve_user_from_headers(
    state: &AppState,
    headers: &HeaderMap,
) -> Option<CurrentUser> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer ").map(str::trim))
        .filter(|s| !s.is_empty());
    let pool = state.pool.as_ref()?;
    let tok = token?;
    sqlx::query_as::<_, CurrentUser>("SELECT id, full_name, username FROM users WHERE api_token = ?")
        .bind(tok)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

/// Mengembalikan 401 jika user tidak terautentikasi. Reserved for protected mutations.
#[allow(dead_code)]
pub fn require_auth(user: Option<CurrentUser>) -> Result<CurrentUser, Response> {
    user.ok_or_else(|| json_error("Unauthorized.", None, StatusCode::UNAUTHORIZED))
}

