// src/backend/dummy.rs

use super::ChatBackend;
use anyhow::Result;
use async_trait::async_trait;

/// A minimal async dummy backend. Useful for offline testing.
pub struct DummyBackend;

impl DummyBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ChatBackend for DummyBackend {
    async fn send_message(&mut self, input: &str) -> Result<String> {
        Ok(format!("(dummy) Echo: {}", input))
    }
}
