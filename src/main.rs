#![allow(dead_code)] // Because all modules except the one used in main() are dead code.

mod d1;
mod d2;

use d2::{solution_1, solution_2};

fn main() {
    println!("Part 1 solution: {}", solution_1());
    println!("Part 2 solution: {}", solution_2());
}
