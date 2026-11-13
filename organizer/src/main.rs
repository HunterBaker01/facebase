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

fn get_embedding(img_path: &PathBuf) -> Result<Vec<Embedding>> {
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
    let embedding: Vec<Embedding> = serde_json::from_str(&ans).context("Failed to parse JSON")?;
    Ok(embedding)
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

    let person_id: i32 = row.get("id");
    add_embedding(client, &person_id, embedding)?;

    remove_file(img_path).context("Failed to delete the registration photos...")?;
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
    let mut client = get_client()?;

    match args.command {
        Mode::Register { image, name } => {
            let path = canonicalize(&image)?;
            let embedding = get_embedding(&path)?;
            register(&mut client, &name, &embedding[0].embedding, &path)?;
            println!("Registered {name}");
        }
        Mode::Recognize { image } => {
            let path = canonicalize(&image)?;
            let embedding = get_embedding(&path)?;
            match recognition(&mut client, &embedding[0].embedding)? {
                Some(name) => println!("Matched: {name}"),
                None => println!("Unknown face"),
            }
        }
    }

    Ok(())
}
