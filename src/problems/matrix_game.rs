// src/problems/matrix_game.rs
//! Решение антагонистической матричной игры в чистых стратегиях
//! Если седловая точка есть — выводим её, иначе сообщаем.

use crate::core::base::*;
use anyhow::Result;

pub struct MatrixGameSolver;

impl MatrixGameSolver {
    pub fn solve(spec: &ProblemSpec) -> Result<DecisionResult> {
        if spec.alternatives.is_empty() || spec.criteria.is_empty() {
            anyhow::bail!("Пустая матрица");
        }

        let m = spec.alternatives.len(); // строки (игрок A)
        let n = spec.criteria.len();      // столбцы (игрок B)

        // матрица стратегий (m x n)
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for alt in &spec.alternatives {
            if alt.values.len() != n {
                anyhow::bail!("Длина значений альтернатив не совпадает");
            }
            matrix.push(alt.values.clone());
        }

        // Минимумы по строкам
        let row_minima: Vec<f64> = matrix.iter().map(|row| row.iter().cloned().fold(f64::INFINITY, f64::min)).collect();
        let maximin = row_minima.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Максимумы по столбцам
        let mut col_maxima: Vec<f64> = Vec::new();
        for j in 0..n {
            let mut mx = f64::NEG_INFINITY;
            for i in 0..m {
                mx = mx.max(matrix[i][j]);
            }
            col_maxima.push(mx);
        }
        let minimax = col_maxima.iter().cloned().fold(f64::INFINITY, f64::min);

        let mut scores = Vec::new();
        let mut chosen = Vec::new();

        if maximin == minimax {
            // Седловая точка есть
            let v = maximin;
            for i in 0..m {
                for j in 0..n {
                    if (matrix[i][j] - v).abs() < 1e-9 {
                        chosen.push(format!("строка={}, столбец={}", spec.alternatives[i].id, spec.criteria[j].id));
                    }
                }
            }
            scores.push(("game_value".to_string(), v));
        } else {
            scores.push(("maximin".to_string(), maximin));
            scores.push(("minimax".to_string(), minimax));

            // .... 
        }

        Ok(DecisionResult {
            chosen,
            scores,
            method: "matrix_game".to_string(),
        })
    }
}
