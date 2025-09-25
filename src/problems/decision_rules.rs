use crate::core::base::{ProblemSpec, DecisionResult};
use anyhow::Result;

pub struct DecisionRulesSolver;

impl DecisionRulesSolver {
    /// criterion: "wald" | "maximax" | "hurwicz" | "savidge" | "laplace"
    pub fn solve(spec: &ProblemSpec, criterion: &str, hurwicz_alpha: Option<f64>) -> Result<DecisionResult> {
        if spec.alternatives.is_empty() || spec.criteria.is_empty() {
            anyhow::bail!("Empty problem");
        }

        let n_alt = spec.alternatives.len();
        let n_crit = spec.criteria.len();

        // собираем матрицу: rows = альтернативы, cols = критерии
        let matrix: Vec<Vec<f64>> = spec.alternatives.iter().map(|a| a.values.clone()).collect();

        let mut scores: Vec<(String, f64)> = Vec::new();

        match criterion {
            "wald" => {
                // минимакс (по каждой альтернативе берём минимум по столбцам)
                for (i, alt) in spec.alternatives.iter().enumerate() {
                    let min_val = matrix[i].iter().cloned().fold(f64::INFINITY, f64::min);
                    scores.push((alt.id.clone(), min_val));
                }
            },
            "maximax" => {
                // максимакс (по каждой альтернативе берём максимум по столбцам)
                for (i, alt) in spec.alternatives.iter().enumerate() {
                    let max_val = matrix[i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    scores.push((alt.id.clone(), max_val));
                }
            },
            "hurwicz" => {
                let alpha = hurwicz_alpha.unwrap_or(0.5);
                for (i, alt) in spec.alternatives.iter().enumerate() {
                    let min_val = matrix[i].iter().cloned().fold(f64::INFINITY, f64::min);
                    let max_val = matrix[i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let score = alpha * max_val + (1.0 - alpha) * min_val;
                    scores.push((alt.id.clone(), score));
                }
            },
            "savidge" => {
                // критерий Сэвиджа-Нигана: строим матрицу потерь (сожалений)
                let mut loss_matrix: Vec<Vec<f64>> = vec![vec![0.0; n_crit]; n_alt];
                for j in 0..n_crit {
                    let col_max = (0..n_alt).map(|i| matrix[i][j]).fold(f64::NEG_INFINITY, f64::max);
                    for i in 0..n_alt {
                        loss_matrix[i][j] = col_max - matrix[i][j];
                    }
                }
                for (i, alt) in spec.alternatives.iter().enumerate() {
                    let max_loss = loss_matrix[i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    scores.push((alt.id.clone(), max_loss));
                }
                // у Сэвиджа минимизируем потери
                scores.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
                let best_val = scores[0].1;
                let chosen: Vec<String> = scores.iter().filter(|(_,v)| (*v - best_val).abs() < 1e-9).map(|(id,_)| id.clone()).collect();
                return Ok(DecisionResult { chosen, scores, method: "savidge".to_string() });
            },
            "laplace" => {
                // усредняем по всем столбцам
                for (i, alt) in spec.alternatives.iter().enumerate() {
                    let avg = matrix[i].iter().sum::<f64>() / n_crit as f64;
                    scores.push((alt.id.clone(), avg));
                }
            },
            _ => anyhow::bail!("Unknown criterion: {}", criterion)
        }

        // для остальных критериев выбираем максимум
        scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        let best_val = scores[0].1;
        let chosen: Vec<String> = scores.iter().filter(|(_,v)| (*v - best_val).abs() < 1e-9).map(|(id,_)| id.clone()).collect();

        Ok(DecisionResult { chosen, scores, method: criterion.to_string() })
    }
}
