// -----------------------------
// src/core/base.rs
// -----------------------------
//! Здесь собраны основные сущности

use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Criterion {
    pub id: String,
    pub weight: Option<f64>, // для многокритериальных задач
    pub maximize: Option<bool>, // true если критерий максимизируемый
}


#[derive(Debug, Deserialize, Clone)]
pub struct Alternative {
    pub id: String,
    pub values: Vec<f64>, // значения по критериям в порядке criteria
}


#[derive(Debug, Deserialize, Clone)]
pub struct ProblemSpec {
    pub alternatives: Vec<Alternative>,
    pub criteria: Vec<Criterion>,
    // Для риск-задач можно указать вероятности состояния (по столбцам альтернативы могут быть векторами ожиданий)
    pub state_probabilities: Option<Vec<f64>>,
}


#[derive(Debug)]
pub struct DecisionResult {
    pub chosen: Vec<String>,
    pub scores: Vec<(String, f64)>,
    pub method: String,
}