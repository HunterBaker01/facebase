use anyhow::Result;
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

use crate::{db, embedding, llm};

#[derive(Clone)]
struct AppState {
    db: Arc<Client>,
    last: Arc<Mutex<Option<(i32, String)>>>,
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
        last: Arc::new(Mutex::new(None)),
    };
    let app = Router::new()
        .route("/api/profile/{id}", get(profile))
        .route("/api/recognize", post(recognize_response))
        .route("/api/register", post(web_register))
        .route("/", get(index))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .with_state(state);

    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
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
) -> Result<Json<RecognizeResponse>, AppError> {
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
            let cached = {
                let last = state.last.lock().unwrap();
                match &*last {
                    Some((last_id, msg)) if *last_id == id => Some(msg.clone()),
                    _ => None,
                }
            };

            let message = match cached {
                Some(msg) => msg,
                None => {
                    let profile = db::get_profile(&state.db, id).await?;
                    let msg = llm::llm(profile).await?;
                    *state.last.lock().unwrap() = Some((id, msg.clone()));
                    msg
                }
            };
            Ok(Json(RecognizeResponse {
                recognized: true,
                message,
            }))
        }
        None => Ok(Json(RecognizeResponse {
            recognized: false,
            message: "You're a guest, yupeee!".to_string(),
        })),
    }
}

async fn web_register(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<String, AppError> {
    let mut image = None;
    let mut name = None;
    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("image") {
            image = Some(field.bytes().await?);
        } else if field.name() == Some("name") {
            name = Some(field.text().await?);
        }
    }
    let bytes = image.ok_or_else(|| anyhow::anyhow!("no image field..."))?;
    let name = name.ok_or_else(|| anyhow::anyhow!("no name field..."))?;

    let file = tempfile::NamedTempFile::new()?;
    tokio::fs::write(file.path(), &bytes).await?;

    let embedding = embedding::get_embedding(file.path()).await?;
    db::register(&state.db, &name, &embedding).await?;

    Ok(format!("Registered {name}"))
}
