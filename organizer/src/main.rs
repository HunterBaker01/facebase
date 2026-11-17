use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

mod db;
mod embedding;
mod llm;

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
    let mut client = db::get_client()?;

    match args.command {
        Mode::Register { image, name } => {
            let path = canonicalize(&image)?;
            let embedding = embedding::get_embedding(&path)?;
            db::register(&mut client, &name, &embedding, &path)?;
            println!("Registered {name}");
        }
        Mode::Recognize { image } => {
            let path = canonicalize(&image)?;
            let embedding = embedding::get_embedding(&path)?;
            match db::recognition(&mut client, &embedding)? {
                Some(person_id) => {
                    let resp = llm::llm(&mut client, &person_id)?;
                    println!("{}", resp);
                }
                None => println!("Unknown face"),
            }
        }
    }

    Ok(())
}
