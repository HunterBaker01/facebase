use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Person {
    identity: String,
}

fn run_recognition(img_path: &str) -> Result<Vec<Person>> {
    let python = Path::new(env!("CARGO_MANIFEST_DIR")).join("../recognizer/recognize.py");
    let output = Command::new("python3").arg(python).arg(img_path).output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    let ans = String::from_utf8(output.stdout)?;
    let p: Vec<Person> = serde_json::from_str(&ans).context("Failed to parse JSON")?;
    Ok(p)
}

fn main() -> Result<()> {
    let path = env::args().nth(1).context("No path provided...")?;
    let result = run_recognition(&path)?;
    println!("{result:?}");
    Ok(())
}
