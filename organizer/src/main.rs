use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::fs::canonicalize;
// use std::fs::remove_file;
use std::path::{Path, PathBuf};

mod db;
mod embedding;
mod llm;
mod server;

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

    Serve {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env");
    dotenvy::from_path(&env_path).context(".env file could not be loaded...")?;
    let client = db::get_client().await?;

    match args.command {
        Mode::Register { image, name } => {
            let path = canonicalize(&image)?;
            let embedding = embedding::get_embedding(&path).await?;
            db::register(&client, &name, &embedding).await?;
            // remove_file(&path).context("Failed to delete the registration photos...")?;
            println!("Registered {name}");
        }
        Mode::Recognize { image } => {
            let path = canonicalize(&image)?;
            let embedding = embedding::get_embedding(&path).await?;
            match db::recognition(&client, &embedding).await? {
                Some(person_id) => {
                    let profile = db::get_profile(&client, person_id).await?;
                    let resp = llm::llm(profile).await?;
                    println!("{}", resp);
                }
                None => println!("Unknown face"),
            }
        }
        Mode::Serve {} => server::run(client).await?,
    }

    Ok(())
}
