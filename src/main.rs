use std::env;
use std::u16;

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(name = "question", default_value = "")]
    text: String,
}


const API_URL: &str = "https://api.openai.com/v1/chat/completions";

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


    println!("{:?}", response);

    Ok(response.status().as_u16())
}

fn main() {
    let args = Args::parse();
    let binding  = env::var("OPENAI_API_KEY").
        expect("OPENAI_API_KEY must be set");
    let api_key = binding.as_str() ;

    let client = Client::new();
    let response = post_question(client, API_URL, api_key, &args.text);

    println!("{:?}", response);
}
