use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use pgvector::Vector;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
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

fn get_embedding(img_path: &PathBuf) -> Result<Vec<Embedding>> {
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
    client: &mut Client,
    person_id: &i32,
    embedding: &Vector,
    img_path: &Path,
) -> Result<()> {
    let path = img_path.to_string_lossy();
    client.execute(
        "INSERT INTO face_embeddings (person_id, embedding, source_image_path) VALUES ($1, $2, $3)",
        &[&person_id, &embedding, &path],
    )?;
    Ok(())
}

fn register(
    client: &mut Client,
    name: &str,
    notes: Option<String>,
    embedding: &Vector,
    img_path: &Path,
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

fn recognition(client: &mut Client, embedding: &Vector) -> Result<Option<String>> {
    let row = client.query_opt(
        "SELECT p.id, p.name, fe.embedding <=> $1 AS distance \
         FROM face_embeddings fe \
         JOIN people p ON p.id = fe.person_id \
         ORDER BY distance LIMIT 1",
        &[&embedding],
    )?;

    let name = match row {
        Some(r) => {
            let distance: f64 = r.get::<_, f64>("distance");
            if distance < 0.4 {
                Some(r.get::<_, String>("name"))
            } else {
                None
            }
        }
        None => None,
    };
    Ok(name)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    image: PathBuf,

    #[arg(short, long)]
    mode: Mode,

    #[arg(short, long, default_value = "None")]
    name: String,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Register,
    Recognize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = canonicalize(args.image)?;
    let embedding = get_embedding(&path);
    let mut client = get_client()?;

    Ok(())
}
