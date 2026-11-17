use anyhow::{Context, Result};
use pgvector::Vector;
use postgres::{Client, NoTls};
use std::fs::remove_file;
use std::path::Path;

pub fn get_client() -> Result<Client> {
    Client::configure()
        .host("localhost")
        .dbname("photos")
        .user(std::env::var("USER")?.as_str())
        .connect(NoTls)
        .context("Wasn't able to connect to the database...")
}

fn add_embedding(client: &mut Client, person_id: &i32, embedding: &Vector) -> Result<()> {
    client.execute(
        "INSERT INTO face_embeddings (person_id, embedding) VALUES ($1, $2)",
        &[&person_id, &embedding],
    )?;
    Ok(())
}

pub fn register(
    client: &mut Client,
    name: &str,
    embedding: &Vector,
    img_path: &Path,
) -> Result<()> {
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

pub fn recognition(client: &mut Client, embedding: &Vector) -> Result<Option<i32>> {
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
