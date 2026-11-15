use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use pgvector::Vector;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::env;
use std::fs::canonicalize;
use std::fs::remove_file;
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

fn get_embedding(img_path: &PathBuf) -> Result<Vector> {
    let python_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../python");
    let script = python_dir.join("embedding.py");
    let output = Command::new("uv")
        .arg("run")
        .arg("--project")
        .arg(&python_dir)
        .arg(script)
        .arg(img_path)
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let ans = String::from_utf8(output.stdout)?;
    let embedding: Embedding = serde_json::from_str(&ans).context("Failed to parse JSON")?;
    Ok(embedding.embedding)
}

fn add_embedding(client: &mut Client, person_id: &i32, embedding: &Vector) -> Result<()> {
    client.execute(
        "INSERT INTO face_embeddings (person_id, embedding) VALUES ($1, $2)",
        &[&person_id, &embedding],
    )?;
    Ok(())
}

fn register(client: &mut Client, name: &str, embedding: &Vector, img_path: &Path) -> Result<()> {
    let row = client.query_one(
        "INSERT INTO people (name) VALUES ($1) \
            RETURNING id",
        &[&name],
    )?;

    let person_id: i32 = row.try_get("id")?;
    add_embedding(client, &person_id, embedding)?;

    remove_file(img_path).context("Failed to delete the registration photos...")?;
    Ok(())
}

fn recognition(client: &mut Client, embedding: &Vector) -> Result<Option<i32>> {
    let row = client.query_opt(
        "SELECT person_id, embedding <=> $1 AS distance \
         FROM face_embeddings \
         ORDER BY distance LIMIT 1",
        &[&embedding],
    )?;

    let person_id = match row {
        Some(r) => {
            let distance: f64 = r.try_get::<_, f64>("distance")?;
            if distance < 0.4 {
                Some(r.try_get::<_, i32>("person_id")?)
            } else {
                None
            }
        }
        None => None,
    };
    Ok(person_id)
}

#[derive(Deserialize)]
struct Block {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct LLMResponse {
    content: Vec<Block>,
}

fn llm(client: &mut Client, id: &i32) -> Result<String> {
    let api_key = env::var("ANTHROPIC_API_KEY").context("LLM Key not set")?;

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let row = client.query_one(
        "SELECT profile \
         FROM people \
         WHERE id = $1",
        &[&id],
    )?;
    let profile: serde_json::Value = row.try_get("profile")?;
    let profile = serde_json::to_string_pretty(&profile)?;

    let resp = http
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2026-10-21")
        .json(&serde_json::json!({
            "model": "claude-opus-5-5",
            "max_tokens": 2000,
            "system": "Write a short personalized greeting based on this persons profile",
            "messages": [{ "role": "user", "content": profile }]
        }))
        .send()?;

    if !resp.status().is_success() {
        bail!("LLM {}: {}", resp.status(), resp.text()?);
    }

    let body: LLMResponse = resp.json()?;
    let text = body
        .content
        .into_iter()
        .filter(|b| b.kind == "text")
        .filter_map(|b| b.text)
        .collect::<String>();

    Ok(text)
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Mode,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Register {
        image: PathBuf,
        #[arg(short, long)]
        name: String,
    },

    Recognize {
        image: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env");
    dotenvy::from_path(&env_path).context(".env file could not be loaded...")?;
    let mut client = get_client()?;

    match args.command {
        Mode::Register { image, name } => {
            let path = canonicalize(&image)?;
            let embedding = get_embedding(&path)?;
            register(&mut client, &name, &embedding, &path)?;
            println!("Registered {name}");
        }
        Mode::Recognize { image } => {
            let path = canonicalize(&image)?;
            let embedding = get_embedding(&path)?;
            match recognition(&mut client, &embedding)? {
                Some(person_id) => println!("Matched: {person_id}"),
                None => println!("Unknown face"),
            }
        }
    }

    Ok(())
}
