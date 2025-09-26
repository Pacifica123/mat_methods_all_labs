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
};

use crate::problems::ahp::AhpSolver;

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
        
        // 1. Многокритериальный анализ
        {
            let result = AhpSolver::solve("data/headphones_ahp.json")?;
            print_result(&result);
        }
        // 1.1 - Сранвение с прерыдущим вариантом
        {
            let spec = core::parser::read_spec("data/example.json")?;
            let result = WeightedSumSolver::solve(&spec)?;
            print_result(&result);
        }

        // // 2. Кластеризация
        // {
        //     // путь к Python можно захардкодить при необходимости
        //     ClusteringSolver::solve(None)?;
        //     println!("Кластеризация завершена: смотри data/answer_for_clustering.txt и dendrogram.png");
        // }

        // // 3. Принятие с учетом риска по критериям
        // let spec = core::parser::read_spec("data/decision_matrix.json")?;
        // let result = DecisionRulesSolver::solve(&spec, "wald", None)?;
        // print_result(&result);
        //
        // let result = DecisionRulesSolver::solve(&spec, "maximax", None)?;
        // print_result(&result);
        //
        // let result = DecisionRulesSolver::solve(&spec, "hurwicz", Some(0.25))?;
        // print_result(&result);
        //
        // let result = DecisionRulesSolver::solve(&spec, "savidge", None)?;
        // print_result(&result);
        //
        // let result = DecisionRulesSolver::solve(&spec, "laplace", None)?;
        // print_result(&result);


        // // 4. Байес, Ферстнер, Ходж
        // let spec = core::parser::read_spec("data/probabilistic_matrix.json")?;
        //
        // let bayes = ProbabilisticRulesSolver::solve(&spec, "bayes", None)?;
        // print_result(&bayes);
        //
        // let ferstner = ProbabilisticRulesSolver::solve(&spec, "ferstner", Some(-0.5))?;
        // print_result(&ferstner);
        //
        // let hodge = ProbabilisticRulesSolver::solve(&spec, "hodge-lehman", Some(0.7))?;
        // print_result(&hodge);

        // // 5. Гермейер
        // let spec = core::parser::read_spec("data/hermeyer_matrix.json")?;
        // let hermeyer = ProbabilisticRulesSolver::solve(&spec, "hermeyer", None)?;
        // print_result(&hermeyer);


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
        Some(other) => anyhow::bail!("Неизвестный метод: {}", other),
        None => unreachable!(),
    }

    Ok(())
}
