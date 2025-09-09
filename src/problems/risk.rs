// -----------------------------
// src/problems/risk.rs
// -----------------------------
//! Модуль с решателем задач, учитывающим риск через ожидаемое значение и критерий Вальда.
//!
//! Решатель `RiskSolver` оценивает альтернативы по ожиданию выплат с учётом вероятностей состояний,
//! а также по минимальному выплачиваемому значению (критерий Вальда).
use crate::core::base::*;
use anyhow::Result;


pub struct RiskSolver;


impl RiskSolver {
pub fn solve(spec: &ProblemSpec) -> Result<DecisionResult> {
// Предположение: альтернативы содержат в values ожидаемые выплаты по состояниям (len == #states)
// state_probabilities должен быть указан и совпадать по длине с values
let probs = spec.state_probabilities.as_ref().ok_or_else(|| anyhow::anyhow!("Probabilities required for risk method"))?;
let ns = probs.len();
if spec.alternatives.iter().any(|a| a.values.len() != ns) { anyhow::bail!("Alternative values length must equal number of states") }


// Ожидаемое значение
let mut ev_scores: Vec<(String, f64)> = spec.alternatives.iter().map(|a|
(a.id.clone(), a.values.iter().zip(probs.iter()).map(|(v,p)| v * p).sum())
).collect();
ev_scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());


// Критерий Вальда (максимин) по состояниям: для каждого альтернативы берем минимальное значение
let mut wal_scores: Vec<(String, f64)> = spec.alternatives.iter().map(|a|
(a.id.clone(), a.values.iter().cloned().fold(f64::INFINITY, f64::min))
).collect();
wal_scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());


// Для наглядности вернём EV как основной
let best = ev_scores[0].1;
let chosen: Vec<String> = ev_scores.iter().filter(|(_,v)| (*v - best).abs() < 1e-9).map(|(id,_)| id.clone()).collect();


// Объединим в один вектор результатов (здесь только EV для простоты)
Ok(DecisionResult{chosen, scores: ev_scores, method: "risk_ev".to_string()})
}
}