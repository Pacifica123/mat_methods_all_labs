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

#[derive(Debug, Clone)]
pub struct FuzzyTerm {
    pub name: String,            // "низкий", "средний", "высокий"
    pub membership: Vec<f64>,    // mu_lj(ui) для всех ui
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuzzyProblem {
    pub universal_set: Vec<f64>,        // U = [160,165,…]
    pub terms: Vec<String>,              // L = ["низкий", "средний", "высокий"]
    pub expert_opinions: Vec<Vec<Vec<u8>>>, 
    // expert_opinions[k][j][i] = b_kji
}
