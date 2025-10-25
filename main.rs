/*  Rust code Ollama API analysis
    Partha Pratim Ray, parthapratimray1986@gmail.com
    30 June, 2025
*/

use std::fs::OpenOptions;
use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::{thread, time};

#[derive(Debug, Deserialize, Serialize)]
struct OllamaResponse {
    model: String,
    #[serde(default)]
    created_at: Option<String>,
    response: String,
    #[serde(default)]
    total_duration: Option<u64>,
    #[serde(default)]
    load_duration: Option<u64>,
    #[serde(default)]
    prompt_eval_count: Option<u64>,
    #[serde(default)]
    prompt_eval_duration: Option<u64>,
    #[serde(default)]
    eval_count: Option<u64>,
    #[serde(default)]
    eval_duration: Option<u64>,
}

const MODEL: &str = "qwen2.5:0.5b"; // llama3.2:1b, gemma3:1b, granite3.1-moe:1b, qwen2.5:0.5b 
const SEED: u64 = 42;
const CSV_FILE: &str = "rust_ollama_log.csv";

const PROMPTS: [(&str, f64); 20] = [
    ("Name any one river in India.", 0.2),
    ("Who wrote the Indian National Anthem?", 0.2),
    ("Translate 'peace' to French.", 0.3),
    ("Suggest a healthy snack for children.", 0.7),
    ("Write a Python function to add two numbers.", 0.5),
    ("Summarize the water cycle in one sentence.", 0.4),
    ("What is the capital of Canada?", 0.2),
    ("Name a fruit that is yellow.", 0.3),
    ("Who is known as the father of computers?", 0.2),
    ("Give me a random English word.", 0.8),
    ("What is the square root of 144?", 0.2),
    ("Suggest a nickname for a friendly dog.", 0.8),
    ("Explain gravity to a child.", 0.5),
    ("Which planet is called the Red Planet?", 0.2),
    ("List any one prime number between 10 and 20.", 0.2),
    ("What comes next in the sequence: 2, 4, 8, 16, ...?", 0.3),
    ("Translate 'thank you' to Spanish.", 0.3),
    ("Tell me a short joke.", 0.9),
    ("Who is the current UN Secretary-General?", 0.2),
    ("Complete: To be, or not to be, ...", 0.4),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set timeout to 600 seconds (10 minutes)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()?;
    let url = "http://localhost:11434/api/generate";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(CSV_FILE)?;
    if file.metadata()?.len() == 0 {
        writeln!(
            file,
            "timestamp,model,prompt,temperature,seed,response,total_duration,load_duration,prompt_eval_count,prompt_eval_duration,eval_count,eval_duration,tokens_per_sec"
        )?;
    }

    for (prompt, temp) in PROMPTS.iter() {
        let req_body = serde_json::json!({
            "model": MODEL,
            "prompt": prompt,
            "stream": false,
            "options": { "temperature": temp, "seed": SEED }
        });

        let start = std::time::Instant::now();
        let resp = client.post(url).json(&req_body).send();
        let _elapsed = start.elapsed();
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: OllamaResponse = resp.json()?;
                    let eval_count = data.eval_count.unwrap_or(0);
                    let eval_duration = data.eval_duration.unwrap_or(1);
                    let tokens_per_sec = if eval_duration > 0 {
                        eval_count as f64 / (eval_duration as f64 / 1_000_000_000.0)
                    } else { 0.0 };
                    writeln!(
                        file,
                        "{},{},{},{},{},{},{},{},{},{},{},{},{:.2}",
                        Utc::now().to_rfc3339(),
                        data.model,
                        prompt,
                        temp,
                        SEED,
                        data.response.replace(",", " ").replace("\n", " "),
                        data.total_duration.unwrap_or(0),
                        data.load_duration.unwrap_or(0),
                        data.prompt_eval_count.unwrap_or(0),
                        data.prompt_eval_duration.unwrap_or(0),
                        eval_count,
                        eval_duration,
                        tokens_per_sec
                    )?;
                    println!("[API] Prompt: {}\nResponse: {}\n---", prompt, data.response.trim());
                } else {
                    println!("[API] Failed status for prompt: {}", prompt);
                }
            }
            Err(e) => {
                println!("[API] Error for prompt '{}': {:?}", prompt, e);
            }
        }
        thread::sleep(time::Duration::from_secs(2)); // avoid flooding
    }
    Ok(())
}
