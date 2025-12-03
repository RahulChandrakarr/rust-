// src/backend/gemini.rs

use super::ChatBackend;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

/// GeminiBackend calls the Generative Language REST API (generateContent).
/// It expects the API key to be provided when constructing the backend.
pub struct GeminiBackend {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiBackend {
    /// Create a new Gemini backend.
    /// - `api_key`: your Gemini/Generative Language API key (or an API key enablement token)
    /// - `model`: model name like "gemini-1.5-flash" or "gemini-2.5-flash". Adjust to what your account supports.
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            model: model.into(),
        }
    }
}

/// Minimal request shapes for the generateContent pattern.
/// We send `contents: [{ role, parts: [{ text }] }]`.
#[derive(Serialize)]
struct ContentPart {
    text: String,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<ContentPart>,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    contents: Vec<Content>,
    // You may add generationConfig here later (temperature, maxOutputTokens, etc.)
}

/// Partial response shapes (we parse only the fields we need).
/// Actual responses vary; we try to extract `candidates[*].content[*].parts[*].text`.
#[derive(Deserialize, Debug)]
struct CandidatePart {
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Option<CandidateContent>,
}

#[derive(Deserialize, Debug)]
struct CandidateContent {
    parts: Option<Vec<CandidatePart>>,
}

#[derive(Deserialize, Debug)]
struct GenerateResponse {
    candidates: Option<Vec<Candidate>>,
    // other fields may exist
}

#[async_trait]
impl ChatBackend for GeminiBackend {
    async fn send_message(&mut self, input: &str) -> Result<String> {
        // Build the request body
        let contents = vec![ Content {
            role: "user".to_string(),
            parts: vec![ ContentPart { text: input.to_string() } ],
        } ];

        let body = GenerateRequest {
            model: self.model.clone(),
            contents,
        };

        // Build endpoint URL.
        // NOTE: Depending on your account/region you may need to use Vertex AI endpoints
        // (projects/locations/.../models:generateContent). This URL is the common REST one.
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model,
            self.api_key
        );

        // POST JSON
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

        // Read the response body exactly once
        let status = resp.status();
        let text_body = resp.text().await.unwrap_or_default();

        // If not success, return helpful error with body text
        if !status.is_success() {
            return Err(anyhow!("Gemini API error {}: {}", status, text_body));
        }

        // Try parse structured response from text
        let parsed: GenerateResponse = serde_json::from_str(&text_body)
            .map_err(|e| anyhow!("Failed to parse Gemini response JSON: {} - body: {}", e, text_body))?;

        if let Some(cands) = parsed.candidates {
            for cand in cands {
                if let Some(content) = cand.content {
                    if let Some(parts) = content.parts {
                        for part in parts {
                            if let Some(t) = part.text {
                                return Ok(t);
                            }
                        }
                    }
                }
            }
        }
        // If parsing didn't produce the expected field
        Err(anyhow!("Gemini response parsed but no text found"))
    }
}
