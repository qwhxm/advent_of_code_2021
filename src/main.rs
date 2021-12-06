#![allow(dead_code)] // Because all modules except the one used in main() are dead code.

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use day6::{solution_1, solution_2};

fn main() {
    println!("Part 1 solution: {}", solution_1());
    println!("Part 2 solution: {}", solution_2());
}
