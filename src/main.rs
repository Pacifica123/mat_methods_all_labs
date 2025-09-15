use anyhow::Result;
use clap::Parser;

mod core;
mod problems;
mod tools;

use core::base::ProblemSpec;
use core::printer::print_result;
use problems::{
    deterministic::DeterministicSolver, 
    multicriteria::WeightedSumSolver, 
    risk::RiskSolver, 
    // nutrition::NutritionSolver,
    clustering::ClusteringSolver
};

#[derive(Parser)]
#[command(author, version, about = "Decision support template", long_about = None)]
struct Cli {
    /// Путь к JSON-файлу с описанием задачи
    input: Option<String>,
    /// Метод: deterministic | multicriteria | risk
    #[arg(short, long, default_value = "deterministic")]
    method: String,
}




fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.method.as_str() {
        "deterministic" | "multicriteria" | "risk" | "nutrition" => {
            let input_path = cli.input.ok_or_else(|| anyhow::anyhow!("Input path required"))?;
            let spec = core::parser::read_spec(&input_path)?;
            let result = match cli.method.as_str() {
                "deterministic" => DeterministicSolver::solve(&spec),
                "multicriteria" => WeightedSumSolver::solve(&spec),
                "risk" => RiskSolver::solve(&spec),
                _ => unreachable!(),
            }?;
            print_result(&result);
        }
        "riskbuild" => {
            // запускаем билдер
            tools::risk_builder::run()?;
        }
        // "nutrition" => {
        //     // NutritionSolver теперь возвращает DecisionResult
        //     let result = NutritionSolver::solve()?;
        //     print_result(&result);
        // }
        "clustering" => {
            // путь к Python-интерпретатору в venv
            // let python_path = Some(
            //     PathBuf::from(env!("CARGO_MANIFEST_DIR")).push
            //     "./tools/lvenv/bin/python"
            // );
            // ClusteringSolver::solve(python_path)?;
            ClusteringSolver::solve(None)?;
            println!("Кластеризация завершена: смотри data/answer_for_clustering.txt и dendrogram.png");
        }
        other => anyhow::bail!("Неизвестный метод: {}", other),
    }
    Ok(())
}
