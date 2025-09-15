// -----------------------------
// src/problems/deterministic.rs
// -----------------------------
//! Модуль с решателем задач методом детерминированного выбора.
//!
//! В этом модуле реализован `DeterministicSolver`, который выбирает
//! оптимальное решение по первому критерию задачи, основываясь
//! на максимизации или минимизации значения.
use crate::core::base::*;
use anyhow::Result;


pub struct DeterministicSolver;


impl DeterministicSolver {
    pub fn solve(spec: &ProblemSpec) -> Result<DecisionResult> {
        // Допущение: одномерный критерий (первый в списке). Берем максимум или минимум по флагу.
        if spec.criteria.is_empty() || spec.alternatives.is_empty() {
            anyhow::bail!("Empty problem")
        }
        let crit = &spec.criteria[0];
        let maximize = crit.maximize.unwrap_or(true);
        let mut scores: Vec<(String, f64)> = spec.alternatives.iter()
            .map(|a| (a.id.clone(), a.values[0]))
            .collect();
        if maximize {
            scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        } else {
            scores.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        }
        let best_val = scores[0].1;
        let chosen: Vec<String> = scores.iter().filter(|(_,v)| (*v - best_val).abs() < 1e-9).map(|(id,_)| id.clone()).collect();

        Ok(DecisionResult{chosen, scores, method: "deterministic".to_string()})
    }
}
