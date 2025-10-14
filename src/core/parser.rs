// -----------------------------
// src/core/parser.rs
// -----------------------------
//! Отвечает за все что касается Input

use crate::core::base::{FuzzyProblem, ProblemSpec};
use anyhow::Result;
use std::fs;


pub fn read_spec(path: &str) -> Result<ProblemSpec> {
    let s = fs::read_to_string(path)?;
    let spec: ProblemSpec = serde_json::from_str(&s)?;
    Ok(spec)
}


// pub fn read_fuzzy_problem(path: &str) -> Result<FuzzyProblem> {
//     let s = std::fs::read_to_string(path)?;
//     let problem: FuzzyProblem = serde_json::from_str(&s)?;
//     Ok(problem)
// }

pub fn read_spec_generic<T: serde::de::DeserializeOwned>(path: &str) -> Result<T> {
    let s = std::fs::read_to_string(path)?;
    let spec: T = serde_json::from_str(&s)?;
    Ok(spec)
}
