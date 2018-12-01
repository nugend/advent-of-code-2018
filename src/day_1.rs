use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.trim().parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve(input:&[i32]) -> i64 { input.iter().map(|&x|x as i64).sum() }
