use crate::core::base::{ProblemSpec, DecisionResult};
use anyhow::Result;

pub struct ProbabilisticRulesSolver;

impl ProbabilisticRulesSolver {
    /// criterion: "bayes" | "ferstner" | "hodge-lehman"
    /// lambda используется для ferstner и hodge-lehman
    pub fn solve(spec: &ProblemSpec, criterion: &str, lambda: Option<f64>) -> Result<DecisionResult> {
        let probs = spec.state_probabilities.as_ref().ok_or_else(|| anyhow::anyhow!("Probabilities required for probabilistic criteria"))?;
        let n_alt = spec.alternatives.len();
        let n_states = probs.len();

        // проверка размеров
        for alt in &spec.alternatives {
            if alt.values.len() != n_states {
                anyhow::bail!("Alternative values length must equal number of states");
            }
        }

        let mut scores: Vec<(String,f64)> = Vec::new();

        match criterion {
            "hermeyer" => {
                for alt in &spec.alternatives {
                    // score = sum(p_j * value_j)
                    let score: f64 = alt.values.iter().zip(probs.iter()).map(|(v,p)| v*p).sum();
                    scores.push((alt.id.clone(), score));
                }
            },
            "bayes" => {
                // среднее взвешенное по вероятностям
                for alt in &spec.alternatives {
                    let score: f64 = alt.values.iter().zip(probs.iter()).map(|(v,p)| v*p).sum();
                    scores.push((alt.id.clone(), score));
                }
            },
            "ferstner" => {
                let lam = lambda.unwrap_or(-0.5);
                for alt in &spec.alternatives {
                    let mean: f64 = alt.values.iter().zip(probs.iter()).map(|(v,p)| v*p).sum();
                    let min_val = *alt.values.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
                    let score = lam*min_val + (1.0-lam)*mean;
                    scores.push((alt.id.clone(), score));
                }
            },
            "hodge-lehman" => {
                let lam = lambda.unwrap_or(0.7);
                for alt in &spec.alternatives {
                    let mean: f64 = alt.values.iter().zip(probs.iter()).map(|(v,p)| v*p).sum();
                    let max_val = *alt.values.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
                    let score = lam*max_val + (1.0-lam)*mean;
                    scores.push((alt.id.clone(), score));
                }
            },
            _ => anyhow::bail!("Unknown criterion: {}", criterion)
        }

        // выбираем максимум
        scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        let best_val = scores[0].1;
        let chosen: Vec<String> = scores.iter().filter(|(_,v)| (*v - best_val).abs() < 1e-9).map(|(id,_)| id.clone()).collect();

        Ok(DecisionResult { chosen, scores, method: criterion.to_string() })
    }
}
