//! Library entry: app router untuk dipakai oleh binary dan integration test.

mod auth;
mod db;
mod handlers;
mod models;
mod openapi;
mod response;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;

pub use state::AppState;

/// Membuat router aplikasi dengan state (pool DB opsional).
pub fn app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let config = utoipa_swagger_ui::Config::from("/docs/openapi.json");
    let swagger = utoipa_swagger_ui::SwaggerUi::new("/")
        .url("/openapi.json", openapi::ApiDoc::openapi())
        .config(config);

    Router::new()
        .route("/", get(handlers::root))
        .nest("/docs", swagger.into())
        .route("/api/users", get(handlers::users_index))
        .route("/api/users/:id", get(handlers::users_show))
        .route("/api/experiences", get(handlers::experiences_index))
        .route("/api/experiences/:id", get(handlers::experiences_show))
        .route("/api/educations", get(handlers::educations_index))
        .route("/api/educations/:id", get(handlers::educations_show))
        .route("/api/skill-categories", get(handlers::skill_categories_index))
        .route("/api/skill-categories/:id", get(handlers::skill_categories_show))
        .route("/api/skills", get(handlers::skills_index))
        .route("/api/skills/:id", get(handlers::skills_show))
        .route("/api/projects", get(handlers::projects_index))
        .route("/api/projects/:id", get(handlers::projects_show))
        .route("/api/blog-posts", get(handlers::blog_posts_index))
        .route("/api/blog-posts/:id", get(handlers::blog_posts_show))
        .route("/api/tags", get(handlers::tags_index))
        .route("/api/tags/:id", get(handlers::tags_show))
        .route("/api/certifications", get(handlers::certifications_index))
        .route("/api/certifications/:id", get(handlers::certifications_show))
        .route("/api/contact", post(handlers::contact_create))
        .route("/api/login", post(handlers::login_post))
        .layer(cors)
        .with_state(state)
}

/// Koneksi database (untuk dipanggil dari main).
pub async fn create_db_pool() -> Result<sqlx::MySqlPool, sqlx::Error> {
    db::create_pool().await
}
