use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.trim().parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn cum_freq(input:&[i32]) -> i64 { input.iter().map(|&x|x as i64).sum() }

#[aoc(day1, part2)]
pub fn repeated_freq(input:&[i32]) -> i64 {
    let mut s = std::collections::HashSet::new();
    let mut freq:i64 = 0;
    s.insert(freq);
    for &x in input.iter().cycle(){
        freq += x as i64;
        if s.contains(&freq) { return freq } else { s.insert(freq) };
    };
    return 0;
}
