// -----------------------------
// src/core/parser.rs
// -----------------------------
//! Отвечает за все что касается Input

use crate::core::base::ProblemSpec;
use anyhow::Result;
use std::fs;


pub fn read_spec(path: &str) -> Result<ProblemSpec> {
    let s = fs::read_to_string(path)?;
    let spec: ProblemSpec = serde_json::from_str(&s)?;
    Ok(spec)
}
