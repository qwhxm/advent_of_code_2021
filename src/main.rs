#![allow(dead_code)] // Because all modules except the one used in main() are dead code.

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

use day16::{solution_1, solution_2};

fn main() {
    println!("Part 1 solution: {}", solution_1());
    println!("Part 2 solution: {}", solution_2());
}
