// -----------------------------
// src/core/printer.rs
// -----------------------------
//! Здесь все что касается Output

use crate::core::base::DecisionResult;


pub fn print_result(res: &DecisionResult) {
    println!("Выбран метод: {}", res.method);
    println!("Значения:");
    for (id, score) in &res.scores {
        println!(" {} -> {:.4}", id, score);
    }
    println!("Выбранная альтернатива(ы):");
    for id in &res.chosen {
        println!(" {}", id);
    }
}
