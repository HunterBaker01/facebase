use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Person {
    identity: String,
}

#[derive(Deserialize, Debug)]
struct Embedding {
    embedding: Vec<f64>,
}

fn get_embedding(img_path: &str) -> Result<Vec<Embedding>> {
    let python = Path::new(env!("CARGO_MANIFEST_DIR")).join("../python/embedding.py");
    let output = Command::new("python3").arg(python).arg(img_path).output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    let ans = String::from_utf8(output.stdout)?;
    let embedding: Vec<Embedding> = serde_json::from_str(&ans).context("Failed to parse JSON")?;
    Ok(embedding)
}

fn main() -> Result<()> {
    let path = env::args().nth(1).context("No path provided...")?;
    let result = get_embedding(&path)?;
    println!("{result:?}");
    Ok(())
}
