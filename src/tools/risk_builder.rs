// -----------------------------
// src/tools/risk_builder.rs
// -----------------------------
use serde::Serialize;
use std::{fs, path::PathBuf, process::Command};
use anyhow::Result;


#[derive(Serialize)]
struct CriterionJson {}


#[derive(Serialize)]
struct AlternativeJson {
    id: String,
    values: Vec<f64>,
}


#[derive(Serialize)]
struct RiskSpecJson {
    criteria: Vec<CriterionJson>,
    state_probabilities: Vec<f64>,
    alternatives: Vec<AlternativeJson>,
}


pub fn run() -> Result<()> {
    // Пример: спрос и вероятности
    let demands = vec![100, 150, 200, 250, 300];
    let probs = vec![0.1, 0.3, 0.3, 0.15, 0.15];


    // цены
    let buy_price = 15.0;
    let sell_price = 22.0;
    let salvage_price = 10.0;


    // формируем матрицу выплат (прибылей)
    let mut alts = Vec::new();
    for &q in &demands {
        let mut values = Vec::new();
        for &n in &demands {
            let sold = q.min(n) as f64;
            let leftover = (q as i32 - n as i32).max(0) as f64;
            let profit = sold * (sell_price - buy_price) + leftover * (salvage_price - buy_price);
            values.push(profit);
        }
        alts.push(AlternativeJson { id: format!("При {}", q), values });
    }


    let spec = RiskSpecJson {
        criteria: vec![],
        state_probabilities: probs,
        alternatives: alts,
    };


    // путь к файлу внутри projectroot/data
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("data");
    fs::create_dir_all(&path)?;
    path.push("data_for_risk.json");


    let s = serde_json::to_string_pretty(&spec)?;
    fs::write(&path, s)?;


    println!("Сгенерирован файл {:?}", path);


    // Запуск cargo run -- data/data_for_risk.json --method risk
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rel_path = PathBuf::from("data/data_for_risk.json");
    let status = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg(rel_path)
    .arg("--method")
    .arg("risk")
    .current_dir(&project_root)
    .status()?;


    if !status.success() {
        anyhow::bail!("cargo run failed");
    }


    Ok(())
}
