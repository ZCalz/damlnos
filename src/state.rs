use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const STATE_PATH: &str = ".damlnos/state.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    /// Index into `EXERCISES` of the exercise the user is currently on.
    pub current: usize,
    /// Per-exercise completion flags. Grows as exercises are completed.
    pub completed: Vec<bool>,
}

impl State {
    pub fn load() -> Result<Self> {
        if !Path::new(STATE_PATH).exists() {
            return Ok(Self::fresh());
        }
        let raw = fs::read_to_string(STATE_PATH)
            .with_context(|| format!("Failed to read {STATE_PATH}"))?;
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {STATE_PATH}"))
    }

    pub fn save(&self) -> Result<()> {
        fs::create_dir_all(".damlnos")?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(STATE_PATH, json)
            .with_context(|| format!("Failed to write {STATE_PATH}"))
    }

    pub fn fresh() -> Self {
        Self { current: 0, completed: vec![] }
    }

    /// Mark exercise at `idx` as complete and advance to the next one.
    pub fn complete(&mut self, idx: usize, total: usize) {
        if self.completed.len() <= idx {
            self.completed.resize(idx + 1, false);
        }
        self.completed[idx] = true;
        if self.current == idx && idx + 1 < total {
            self.current = idx + 1;
        }
    }

    pub fn is_complete(&self, idx: usize) -> bool {
        self.completed.get(idx).copied().unwrap_or(false)
    }
}
