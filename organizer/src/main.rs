use anyhow::{Context, Result, bail};
use pgvector::Vector;
use postgres::{Client, NoTls, Row};
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Embedding {
    embedding: Vector,
}

fn get_client() -> Result<Client> {
    Client::configure()
        .host("localhost")
        .dbname("photos")
        .user(std::env::var("USER")?.as_str())
        .connect(NoTls)
        .context("Wasn't able to connect to the database...")
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

fn add_embedding(
    mut client: Client,
    person_id: &i32,
    embedding: &Vector,
    img_path: &str,
) -> Result<()> {
    client.execute(
        "INSERT INTO face_embeddings (person_id, embedding, source_image_path) VALUES ($1, $2, $3)",
        &[&person_id, &embedding, &img_path],
    )?;
    Ok(())
}

fn register(
    mut client: Client,
    name: &str,
    notes: Option<String>,
    embedding: &Vector,
    img_path: &str,
) -> Result<()> {
    let row = client.query_one(
        "INSERT INTO people (name, notes) VALUES ($1, $2) \
            ON CONFLICT (name) DO UPDATE SET notes = EXCLUDED.notes \
            RETURNING id",
        &[&name, &notes.unwrap_or(String::from("None"))],
    )?;

    let person_id: i32 = row.get("id");
    add_embedding(client, &person_id, embedding, img_path)?;
    Ok(())
}

fn main() -> Result<()> {
    let path = env::args().nth(1).context("No path provided...")?;
    let result = get_embedding(&path)?;
    let embedding = &result[0].embedding;
    let client = get_client()?;
    let person_id = 15;
    register(client, &person_id, embedding, &path)?;
    Ok(())
}
