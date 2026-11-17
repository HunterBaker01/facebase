use anyhow::Result;
use axum::{Router, routing::get};

async fn hello_world() -> &'static str {
    "Hello World!"
}

pub async fn run() -> Result<()> {
    let app = Router::new().route("/hello", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
