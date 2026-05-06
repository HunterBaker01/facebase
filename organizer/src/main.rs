use anyhow::{Context, Result, bail};
use pgvector::Vector;
use postgres::{Client, NoTls};
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
    client: &mut Client,
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
    client: &mut Client,
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

fn main() -> Result<()> {
    let path = env::args().nth(1).context("No path provided...")?;
    let path2 = env::args().nth(2).context("Nops")?;
    let result = get_embedding(&path)?;
    let result2 = get_embedding(&path2)?;
    let embedding = &result[0].embedding;
    let embedding2 = &result2[0].embedding;
    let mut client = get_client()?;
    register(
        &mut client,
        "Hunter",
        Some(String::from("Beast")),
        embedding,
        &path,
    )?;
    let name = recognition(&mut client, embedding2)?;
    println!("{name:?}");
    Ok(())
}
