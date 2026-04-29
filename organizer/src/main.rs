use anyhow::{Context, Result, bail};
use serde_json;
use std::env;
use std::path::Path;
use std::process::Command;

fn run_recognition(img_path: &str) -> Result<String> {
    let python = Path::new(env!("CARGO_MANIFEST_DIR")).join("../recognizer/recognize.py");
    let output = Command::new("python3").arg(python).arg(img_path).output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    let ans = String::from_utf8(output.stdout)?;
    Ok(ans)
}

fn main() -> Result<()> {
    let path = env::args().nth(1).context("No path provided...")?;
    let result = run_recognition(&path)?;
    println!("{result}");
    Ok(())
}
