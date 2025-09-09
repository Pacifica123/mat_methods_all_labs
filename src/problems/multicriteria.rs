// -----------------------------
// src/problems/multicriteria.rs
// -----------------------------
//! Модуль с решателем задач методом взвешенной суммы.
//!
//! В этом модуле реализован `WeightedSumSolver`, который вычисляет
//! итоговую оценку альтернатив как взвешенную сумму значений по критериям.
//! Веса нормализуются так, чтобы сумма была равна 1.
use crate::core::base::*;
use anyhow::Result;


pub struct WeightedSumSolver;


impl WeightedSumSolver {
pub fn solve(spec: &ProblemSpec) -> Result<DecisionResult> {
let m = spec.criteria.len();
if m == 0 { anyhow::bail!("No criteria") }
// Соберём веса (по умолчанию равномерно)
let weights: Vec<f64> = spec.criteria.iter().map(|c| c.weight.unwrap_or(1.0)).collect();
// Нормализуем веса
let sw: f64 = weights.iter().sum();
let weights: Vec<f64> = weights.into_iter().map(|w| w / sw).collect();


let mut scores: Vec<(String, f64)> = Vec::new();
for alt in &spec.alternatives {
if alt.values.len() != m { anyhow::bail!("Alternative values length mismatch") }
let s = alt.values.iter().enumerate().map(|(i,v)| v * weights[i]).sum();
scores.push((alt.id.clone(), s));
}
scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
let best = scores[0].1;
let chosen: Vec<String> = scores.iter().filter(|(_,v)| (*v - best).abs() < 1e-9).map(|(id,_)| id.clone()).collect();
Ok(DecisionResult{chosen, scores, method: "multicriteria_weighted_sum".to_string()})
}
}