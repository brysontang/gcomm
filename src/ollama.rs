use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

fn build_system_prompt() -> &'static str {
    r#"You are a commit message assistant. Your job is to write clear, structured Git commit messages based on a given diff.

Use this format:

<Title (max 70 chars)>

Added
  - Description

Changed
  - Description

Removed
  - Description

Only include sections that are present in the changes. Avoid excessive verbosity. Write messages that are readable in Git logs.

Example:

Add user analytics and cleanup legacy settings

Added
  - Event tracking for signup and login
  - Analytics integration with Mixpanel

Changed
  - Renamed `userFlow` to `userJourney`
  - Updated settings panel layout

Removed
  - Deprecated `oldTracking.js`
"#
}

pub fn query_ollama(diff: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!("Write a structured Git commit message for the following diff:\n\n{}", diff);

    let client = Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&OllamaRequest {
            model,
            prompt: &prompt,
            system: build_system_prompt(),
            stream: false,
        })
        .send()?
        .json::<OllamaResponse>()?;

    Ok(res.response.trim().to_string())
}
