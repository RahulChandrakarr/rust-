// src/backend/mod.rs

// public submodules (create these files)
pub mod dummy;
pub mod gemini;

use async_trait::async_trait;
use anyhow::Result;

/// Async trait that every backend should implement.
/// send_message returns a Result<String> because network calls can fail.
#[async_trait]
pub trait ChatBackend: Send + Sync {
    async fn send_message(&mut self, input: &str) -> Result<String>;
}
