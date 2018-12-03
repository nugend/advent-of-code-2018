use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator<'a>(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_owned()).collect()
}
#[aoc(day2, part2)]
pub fn neighbors(input: &[String]) -> String {
    let mut input_iter = input.iter();
    while let Some(id1) = input_iter.next() {
        for id2 in input_iter.clone() {
            let cpairs = id1.chars().zip(id2.chars());
            let mut differ = 0;
            let mut differ_idx = 0;
            for (idx, (c1, c2)) in cpairs.enumerate() {
                if let true = c1 != c2 {
                    differ += 1;
                    differ_idx = idx
                };
            }
            if differ == 1 {
                let mut rval = id1.to_string();
                let _ = rval.remove(differ_idx);
                return rval;
            }
        }
    }
    "".to_string()
}

#[aoc(day2, part1)]
pub fn checksum(input: &[String]) -> u32 {
    let mut twice = 0;
    let mut thrice = 0;
    for id in input {
        let mut letters = std::collections::HashMap::with_capacity(id.len());

        for ch in id.chars() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }
        let mut twice_seen = false;
        let mut thrice_seen = false;
        for occurrences in letters.values() {
            match occurrences {
                2 if !twice_seen => {
                    twice += 1;
                    if thrice_seen {
                        break;
                    } else {
                        twice_seen = true
                    };
                }
                3 if !thrice_seen => {
                    thrice += 1;
                    if twice_seen {
                        break;
                    } else {
                        thrice_seen = true
                    };
                }
                _ => (),
            }
        }
    }
    return twice * thrice;
}
