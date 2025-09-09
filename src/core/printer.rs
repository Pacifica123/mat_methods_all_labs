// -----------------------------
// src/core/printer.rs
// -----------------------------
//! Здесь все что касается Output

use crate::core::base::DecisionResult;


pub fn print_result(res: &DecisionResult) {
println!("Method: {}", res.method);
println!("Scores:");
for (id, score) in &res.scores {
println!(" {} -> {:.4}", id, score);
}
println!("Chosen alternative(s):");
for id in &res.chosen {
println!(" {}", id);
}
}