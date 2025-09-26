use anyhow::Result;
use crate::core::base::DecisionResult;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct AhpInput {
    criteria_comparison: Vec<Vec<f64>>, // матрица n×n
    criteria: Vec<String>,              // названия критериев
    alternatives: Vec<AhpAlternative>,  // альтернативы с их значениями
}

#[derive(Debug, Deserialize)]
struct AhpAlternative {
    id: String,
    values: Vec<f64>, // (в том же порядке, что criteria)
}

pub struct AhpSolver;

impl AhpSolver {
    pub fn solve(input_path: &str) -> Result<DecisionResult> {
        let s = fs::read_to_string(input_path)?;
        let input: AhpInput = serde_json::from_str(&s)?;

        let n = input.criteria.len();
        let m = input.alternatives.len();

        // 1. Проверка ввода
        if input.criteria_comparison.len() != n {
            anyhow::bail!("Матрица критериев должна быть {}×{}", n, n);
        }

        // 2. Нормирование строк
        let mut col_sums = vec![0.0; n];
        for j in 0..n {
            for i in 0..n {
                col_sums[j] += input.criteria_comparison[i][j];
            }
        }
        let mut normalized = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                normalized[i][j] = input.criteria_comparison[i][j] / col_sums[j];
            }
        }
        let mut crit_weights = vec![0.0; n];
        for i in 0..n {
            let sum_row: f64 = normalized[i].iter().sum();
            crit_weights[i] = sum_row / n as f64;
        }
        println!("Критерии и их веса (AHP):");
        for (i, w) in crit_weights.iter().enumerate() {
            println!("  {} -> {:.4}", input.criteria[i], w);
        }

        // 3. Проверка согласованности
        let mut lambda_vec = vec![0.0; n];
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += input.criteria_comparison[i][j] * crit_weights[j];
            }
            lambda_vec[i] = s / crit_weights[i];
        }
        let lambda_max: f64 = lambda_vec.iter().sum::<f64>() / n as f64;
        let ci = (lambda_max - n as f64) / (n as f64 - 1.0);
        let ri = choose_index(n);
        let cr = if ri > 0.0 { ci / ri } else { 0.0 };

        // 4. Локальные приоритеты альтернатив 
        let mut local = vec![vec![0.0; m]; n];
        for crit in 0..n {
            let mut sum_col = 0.0;
            for alt in 0..m {
                sum_col += input.alternatives[alt].values[crit];
            }
            for alt in 0..m {
                local[crit][alt] = input.alternatives[alt].values[crit] / sum_col;
            }
        }

        // 5. Синтез: глобальные веса альтернатив
        let mut global_scores = vec![0.0; m];
        for alt in 0..m {
            for crit in 0..n {
                global_scores[alt] += crit_weights[crit] * local[crit][alt];
            }
        }

        // 6. Сортировка и результат
        let mut scores: Vec<(String, f64)> = input
            .alternatives
            .iter()
            .enumerate()
            .map(|(i, a)| (a.id.clone(), global_scores[i]))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let best_val = scores[0].1;
        let chosen: Vec<String> = scores
            .iter()
            .filter(|(_, v)| (*v - best_val).abs() < 1e-9)
            .map(|(id, _)| id.clone())
            .collect();

        Ok(DecisionResult {
            chosen,
            scores,
            method: format!(
                "AHP (λ_max={:.3}, CI={:.3}, CR={:.3})",
                lambda_max, ci, cr
            ),
        })
    }
}

/// Случайный индекс (таблица Саати)
fn choose_index(n: usize) -> f64 {
    match n {
        1 | 2 => 0.0,
        3 => 0.58,
        4 => 0.90,
        5 => 1.12,
        6 => 1.24,
        7 => 1.32,
        8 => 1.41,
        9 => 1.45,
        10 => 1.49,
        _ => 1.49,
    }
}
