use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| i32::from(l)).collect()
}

#[aoc(day1, part1)]
pub fn solve(input:&[i32]) -> i64 { input.sum() }
