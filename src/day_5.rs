use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn polymerization(input: &str) -> usize {
    let mut last_str = input.to_string();
    loop {
        let (mut s, last) = last_str.chars().fold((String::new(),' '),{|(s,p),x|
                             if p == ' ' {
                                 (s,x)
                             } else if p != x && (p.to_ascii_uppercase() == x || x.to_ascii_uppercase() == p) {
                                 (s,' ')
                             } else {
                                 let mut s = s.clone();
                                 s.push(p);
                                 (s,x)
                             }
        });
        s.push(last);
        if last_str == s { return s.trim().len() } else { last_str = s };
    }
}

#[aoc(day5, part2)]
pub fn shortest_polymer_variant(input: &str) -> usize {
    let mut shortest = (std::usize::MAX,' ');
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut sample = input.to_string();
        sample.retain(|x| x != c && x != c.to_ascii_uppercase());
        let len = polymerization(&sample);
        shortest = if len < shortest.0 {
            (len,c)
        } else { shortest }
    }
    return shortest.0
}
