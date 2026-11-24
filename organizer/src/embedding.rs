use anyhow::{Context, Result, bail};
use pgvector::Vector;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Deserialize, Debug)]
struct Embedding {
    embedding: Vector,
}

pub async fn get_embedding(img_path: &Path) -> Result<Vector> {
    let python_dir =
        PathBuf::from(std::env::var("PYTHON_DIR").unwrap_or_else(|_| "./python".into()));
    let script = python_dir.join("embedding.py");
    let output = Command::new("uv")
        .arg("run")
        .arg("--project")
        .arg(&python_dir)
        .arg(script)
        .arg(img_path)
        .output()
        .await?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let ans = String::from_utf8(output.stdout)?;
    let embedding: Embedding = serde_json::from_str(&ans).context("Failed to parse JSON")?;
    Ok(embedding.embedding)
}
