use anyhow::Result;
use clap::Parser;

mod core;
mod problems;
mod tools;

use core::printer::print_result;
use problems::{
    deterministic::DeterministicSolver, 
    multicriteria::WeightedSumSolver, 
    risk::RiskSolver, 
    clustering::ClusteringSolver,
    decision_rules::DecisionRulesSolver,
    bayes_rules::ProbabilisticRulesSolver,
    matrix_game::MatrixGameSolver,
    fuzzy::FuzzySolver,

};
use crate::core::base::{FuzzyProblem, FuzzyTerm};


#[derive(Parser)]
#[command(author, version, about = "Decision support template", long_about = None)]
struct Cli {
    /// Путь к JSON-файлу с описанием задачи
    input: Option<String>,
    /// Метод: deterministic | multicriteria | risk | clustering
    #[arg(short, long)]
    method: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.method.is_none() {

        // 6. Модели конфликтных ситуаций (L4):
        {
            let spec1 = core::parser::read_spec("data/data_for_conflictgame1.json")?;
            let spec2 = core::parser::read_spec("data/data_for_conflictgame2.json")?;
            let spec3 = core::parser::read_spec("data/data_for_conflictgame3.json")?;
            let result1 = MatrixGameSolver::solve(&spec1)?;
            let result2 = MatrixGameSolver::solve(&spec2)?;
            let result3 = MatrixGameSolver::solve(&spec3)?;
            print_result(&result1);
            print_result(&result2);
            print_result(&result3);
        }
        println!("--------------------------\n");
        // Принятие решений в условиях нечеткости (L5)
        {
            // 1. Первая подзадача: интервал и min-тнорм
            let interval_result = FuzzySolver::solve(Some("src/tools/lvenv/bin/python"))?;
            println!("Первая задача L5 выполнена:");
            println!("Chosen: {:?}", interval_result.chosen);
            println!("Scores: {:?}", interval_result.scores);

            // 2. Вторая подзадача: термы «низкий», «средний», «высокий»
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            let fuzzy_input_path = format!("{}/data/data_for_fuzzy_terms.json", manifest_dir);
            
            // Загружаем JSON с экспертными оценками
            let fuzzy_problem: FuzzyProblem = core::parser::read_spec_generic(&fuzzy_input_path)?;
            
            // Вычисляем функции принадлежности
            let terms = FuzzySolver::solve_term(&fuzzy_problem)?;
            
            // Сохраняем и строим графики через Python
            FuzzySolver::export_and_plot(&terms, &fuzzy_problem.universal_set, "src/tools/lvenv/bin/python")?;
            println!("Вторая задача L5 выполнена: построены функции принадлежности термов (см. tools/output/)");
        }


        return Ok(());
    }

    // ---- Старый режим через CLI ----
    match cli.method.as_deref() {
        Some("deterministic") | Some("multicriteria") | Some("risk") => {
            let input_path = cli.input.ok_or_else(|| anyhow::anyhow!("Input path required"))?;
            let spec = core::parser::read_spec(&input_path)?;
            let result = match cli.method.as_deref() {
                Some("deterministic") => DeterministicSolver::solve(&spec),
                Some("multicriteria") => WeightedSumSolver::solve(&spec),
                Some("risk") => RiskSolver::solve(&spec),
                _ => unreachable!(),
            }?;
            print_result(&result);
        }
        Some("riskbuild") => {
            tools::risk_builder::run()?;
        }
        Some("clustering") => {
            ClusteringSolver::solve(None)?;
            println!("Кластеризация завершена: см.: data/answer_for_clustering.txt и dendrogram.png");
        }
        Some("matrix") => {
            let input_path = cli.input.ok_or_else(|| anyhow::anyhow!("Input path required"))?;
            let spec = core::parser::read_spec(&input_path)?;
            let result = MatrixGameSolver::solve(&spec)?;
            print_result(&result);
        }

        Some(other) => anyhow::bail!("Неизвестный метод: {}", other),
        None => unreachable!(),
    }

    Ok(())
}
