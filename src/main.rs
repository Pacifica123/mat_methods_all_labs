use anyhow::Result;
use clap::Parser;

mod core;
mod problems;

use core::base::ProblemSpec;
use core::printer::print_result;
use problems::{deterministic::DeterministicSolver, multicriteria::WeightedSumSolver, risk::RiskSolver};

#[derive(Parser)]
#[command(author, version, about = "Decision support template", long_about = None)]
struct Cli {
    /// Путь к JSON-файлу с описанием задачи
    input: String,
    /// Метод: deterministic | multicriteria | risk
    #[arg(short, long, default_value = "deterministic")]
    method: String,
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    let spec = core::parser::read_spec(&cli.input)?;

    let result = match cli.method.as_str() {
        "deterministic" => DeterministicSolver::solve(&spec),
        "multicriteria" => WeightedSumSolver::solve(&spec),
        "risk" => RiskSolver::solve(&spec),
        other => anyhow::bail!("Unknown method: {}", other),
    }?;

    print_result(&result);
    Ok(())
}