// src/main.rs

mod backend;

use backend::dummy::DummyBackend;
use backend::gemini::GeminiBackend;
use backend::ChatBackend;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use colored::*;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env if present
    dotenv().ok();

    println!("{}", "CLI AI Agent (async)".green());
    println!("Commands: /exit, /mode dummy, /mode gemini, /reset");
    println!("Type a message to send to the current backend.\n");

    // Start with DummyBackend by default
    let mut backend: Box<dyn ChatBackend> = Box::new(DummyBackend::new());
    println!("{}", "Using backend: dummy".yellow());

    // Async stdin reader
    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        // Handle commands beginning with '/'
        if input.starts_with('/') {
            let parts: Vec<&str> = input.split_whitespace().collect();
            match parts.as_slice() {
                ["/exit"] | ["/quit"] => {
                    println!("{}", "Goodbye!".green());
                    break;
                }
                ["/mode", "dummy"] => {
                    backend = Box::new(DummyBackend::new());
                    println!("{}", "Switched to backend: dummy".yellow());
                    continue;
                }
                ["/mode", "gemini"] => {
                    // Try to create Gemini backend using env vars GEMINI_API_KEY and optional GEMINI_MODEL
                    match env::var("GEMINI_API_KEY") {
                        Ok(key) => {
                            let model = env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.5-flash".to_string());
                            backend = Box::new(GeminiBackend::new(key, model));
                            println!("{}", "Switched to backend: gemini".yellow());
                        }
                        Err(_) => {
                            println!("{}", "GEMINI_API_KEY not set in environment. Set it in .env or as env var.".red());
                        }
                    }
                    continue;
                }
                ["/reset"] => {
                    // Recreate current backend as a fresh instance (simple way to reset state).
                    // If it was Gemini, re-create Gemini backend using env vars; else recreate Dummy.
                    // (We can't detect type easily from Box<dyn>, so just recreate Dummy for simplicity.)
                    backend = Box::new(DummyBackend::new());
                    println!("{}", "Backend reset to fresh Dummy instance.".yellow());
                    continue;
                }
                _ => {
                    println!("{}", "Unknown command.".red());
                    continue;
                }
            }
        }

        // Regular message -> send to backend asynchronously
        match backend.send_message(input).await {
            Ok(reply) => {
                println!("{} {}", "AI:".cyan(), reply);
            }
            Err(e) => {
                println!("{} {}", "AI error:".red(), e);
            }
        }
    }

    Ok(())
}
