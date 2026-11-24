use anyhow::{Context, Result};
use pgvector::Vector;
use tokio_postgres::{Client, NoTls};

pub async fn get_client() -> Result<Client> {
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL not set in .env")?;

    let (client, connection) = tokio_postgres::connect(&url, NoTls)
        .await
        .context("Wasn't able to connect to the database...")?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("database connection error: {e}");
        }
    });

    Ok(client)
}

async fn add_embedding(client: &Client, person_id: &i32, embedding: &Vector) -> Result<()> {
    client
        .execute(
            "INSERT INTO face_embeddings (person_id, embedding) VALUES ($1, $2)",
            &[&person_id, &embedding],
        )
        .await?;
    Ok(())
}

pub async fn register(client: &Client, name: &str, embedding: &Vector) -> Result<()> {
    let row = client
        .query_one(
            "INSERT INTO people (name) VALUES ($1) \
            RETURNING id",
            &[&name],
        )
        .await?;

    let person_id: i32 = row.try_get("id")?;
    add_embedding(client, &person_id, embedding).await?;

    Ok(())
}

pub async fn recognition(client: &Client, embedding: &Vector) -> Result<Option<i32>> {
    let row = client
        .query_opt(
            "SELECT person_id, embedding <=> $1 AS distance \
         FROM face_embeddings \
         ORDER BY distance LIMIT 1",
            &[&embedding],
        )
        .await?;

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

pub async fn get_profile(client: &Client, id: i32) -> Result<serde_json::Value> {
    let row = client
        .query_one(
            "SELECT profile \
         FROM people \
         WHERE id = $1",
            &[&id],
        )
        .await?;
    let profile: serde_json::Value = row.try_get("profile")?;
    Ok(profile)
}
