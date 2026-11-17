use anyhow::{Context, Result, bail};
use postgres::Client;
use serde::Deserialize;
use std::env;

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

pub fn llm(client: &mut Client, id: &i32) -> Result<String> {
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
