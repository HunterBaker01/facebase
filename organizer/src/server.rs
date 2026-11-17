use anyhow::Result;
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::{db, embedding, llm};

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

#[derive(Serialize)]
struct RecognizeResponse {
    recognized: bool,
    message: String,
}

pub async fn run(client: Client) -> Result<()> {
    let state = AppState {
        db: Arc::new(client),
    };
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/api/profile/{id}", get(profile))
        .route("/api/recognize", post(recognize_response))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
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

async fn recognize_response(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<String, AppError> {
    let mut image = None;
    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("image") {
            image = Some(field.bytes().await?);
        }
    }
    let bytes = image.ok_or_else(|| anyhow::anyhow!("no image field..."))?;

    let file = tempfile::NamedTempFile::new()?;
    tokio::fs::write(file.path(), &bytes).await?;

    let embedding = embedding::get_embedding(file.path()).await?;

    match db::recognition(&state.db, &embedding).await? {
        Some(id) => {
            let profile = db::get_profile(&state.db, id).await?;
            let message = llm::llm(profile).await?;
            Ok(message)
        }
        None => Ok("You're a guest, yuppee".to_string()),
    }
}
