use anyhow::Result;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router, http::StatusCode, routing::get};
use std::sync::Arc;
use tokio_postgres::Client;

use crate::db;

async fn hello_world() -> &'static str {
    "Hello World!"
}

#[derive(Clone)]
struct AppState {
    db: Arc<Client>,
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("request failed: {:#}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong... ",
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn run(client: Client) -> Result<()> {
    let state = AppState {
        db: Arc::new(client),
    };
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/api/profile/{id}", get(profile))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn profile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let profile = db::get_profile(&state.db, id).await?;
    Ok(Json(profile))
}
