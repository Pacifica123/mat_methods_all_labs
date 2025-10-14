// src/problems/fuzzy.rs
use anyhow::Result;
use crate::core::base::{DecisionResult, FuzzyProblem, FuzzyTerm};
use serde::{Deserialize, Serialize};
use std::fs;
// use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;
use std::f64::consts::PI;
use std::io::Write;


#[derive(Debug, Deserialize, Serialize)]
struct Interval {
    low: f64,
    high: f64,
}

#[derive(Debug, Deserialize)]
struct FuzzyInput {
    p: Interval,
    q: Interval,
    a1: Option<f64>, // параметр эксперта, по умолчанию 0.2
}

#[derive(Debug, Serialize)]
struct FuzzyOutput {
    r: Interval,
    a1: f64,
    mu_true_low: f64,
    mu_true_high: f64,
    mu_false_low: f64,
    mu_false_high: f64,
}

pub struct FuzzySolver;

impl FuzzySolver {
    pub fn mu_true(x: f64, a1: f64) -> f64 {
        if x < a1 || x > 1.0 {
            return 0.0_f64.max(((0.5*(1.0 + (PI/2.0) * ((2.0*x - 1.0 - a1)/(1.0 - a1)))) ) ).min(1.0);
        }
        let v = 0.5 * (1.0 + (PI/2.0) * ((2.0*x - 1.0 - a1) / (1.0 - a1)));
        v.max(0.0).min(1.0)
    }

    pub fn mu_false(x: f64, a1: f64) -> f64 {
        if x < 0.0 || x > 1.0 - a1 {
            return 0.0_f64.max( ((0.5*(1.0 + (PI/2.0) * ((1.0 - a1 - 2.0*x)/(1.0 - a1)))) ) ).min(1.0);
        }
        let v = 0.5 * (1.0 + (PI/2.0) * ((1.0 - a1 - 2.0*x) / (1.0 - a1)));
        v.max(0.0).min(1.0)
    }

    /// python_path: Some("/path/to/venv/bin/python") or None -> "python3"
    pub fn solve(python_path: Option<&str>) -> Result<DecisionResult> {
        // читаем
        let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        in_path.push("data/data_for_fuzzy.json");
        let raw = fs::read_to_string(&in_path)?;
        let input: FuzzyInput = serde_json::from_str(&raw)?;

        let a1 = input.a1.unwrap_or(0.2);

        // конъюнкция (пересечение)
        let r_low = input.p.low.min(input.q.low);
        let r_high = input.p.high.min(input.q.high);

        // считаем значения mu
        let mu_t_low = FuzzySolver::mu_true(r_low, a1);
        let mu_t_high = FuzzySolver::mu_true(r_high, a1);
        let mu_f_low = FuzzySolver::mu_false(r_low, a1);
        let mu_f_high = FuzzySolver::mu_false(r_high, a1);

        // пишем в out для графика
        let out = FuzzyOutput {
            r: Interval { low: r_low, high: r_high },
            a1,
            mu_true_low: mu_t_low,
            mu_true_high: mu_t_high,
            mu_false_low: mu_f_low,
            mu_false_high: mu_f_high,
        };

        let mut out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        out_path.push("data/answer_for_fuzzy.json");
        fs::write(&out_path, serde_json::to_string_pretty(&out)?)?;

        // call python 
        let python_bin = python_path.unwrap_or("python3");
        let mut script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        script_path.push("src/tools/plot_fuzzy.py");

        let output = Command::new(python_bin)
            .arg(script_path)
            .arg(&out_path)
            .output()?;
        if !output.status.success() {
            eprintln!("Python error: {}", String::from_utf8_lossy(&output.stderr));
            anyhow::bail!("Python fuzzy plot script failed");
        }


        // возвращаем DecisionResult
        let scores = vec![
            ("r_low".to_string(), r_low),
            ("r_high".to_string(), r_high),
        ];
        let chosen = vec![
            format!("answer_file: {}", out_path.to_string_lossy()),
            format!("plot: fuzzy_plot.png"),
        ];

        Ok(DecisionResult {
            chosen,
            scores,
            method: "fuzzy_interval_min_tnorm".to_string(),
        })
    }


    pub fn solve_term(problem: &FuzzyProblem) -> Result<Vec<FuzzyTerm>> {
        let k = problem.expert_opinions.len() as f64;
        let m = problem.terms.len();
        let n = problem.universal_set.len();

        let mut terms: Vec<FuzzyTerm> = Vec::new();

        for j in 0..m {
            let mut membership: Vec<f64> = Vec::new();
            for i in 0..n {
                // усредняем по экспертам
                let sum: u8 = problem.expert_opinions.iter()
                    .map(|exp| exp[j][i])
                    .sum();
                membership.push(sum as f64 / k);
            }
            terms.push(FuzzyTerm { 
                name: problem.terms[j].clone(), 
                membership 
            });
        }

        Ok(terms)
    }

    pub fn export_and_plot(terms: &Vec<FuzzyTerm>, universal_set: &Vec<f64>, python_path: &str) -> Result<()> {
        let data = serde_json::json!({
            "universal_set": universal_set,
            "terms": terms.iter().map(|t| {
                serde_json::json!({"name": t.name, "membership": t.membership})
            }).collect::<Vec<_>>()
        });

        let path = "data/fuzzy_result.json";
        let mut file = std::fs::File::create(path)?;
        write!(file, "{}", serde_json::to_string_pretty(&data)?)?;

        // Запуск Python
        Command::new(python_path)
            .arg("src/tools/draw_fuzzy_terms.py")
            .status()?;
        Ok(())
    }
}
