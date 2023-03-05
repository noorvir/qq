use std::env;
use std::u16;

use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(name = "question", default_value = "")]
    text: String,
}

#[derive(Debug, Deserialize)]
struct Answer {
    id: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}


fn post_question(client: Client, url: &str, api_key: &str, question: &str) -> Result<u16, reqwest::Error> {
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": question}],
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()?;

    let status = response.status().as_u16();
    let answer: Answer = response.json()?;

    let out = format!(
        "Question: \n{}\n\n\
         Answer:   {}\n\
         ", question, answer.choices[0].message.content);

    println!("{}", out);

    Ok(status)
}

fn main() {
    let args = Args::parse();
    let binding = env::var("OPENAI_API_KEY").
        expect("OPENAI_API_KEY must be set");
    let api_key = binding.as_str();

    let client = Client::new();
    let _ = post_question(client, API_URL, api_key, &args.text);
}
