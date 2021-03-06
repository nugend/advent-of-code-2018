use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Dim {
    w: u16,
    h: u16,
}

#[derive(Debug)]
struct Bound {
    pos: Pos,
    dim: Dim,
}

#[derive(Debug)]
pub struct Claim {
    id: u16,
    bound: Bound,
}

impl Bound {
    pub fn intersection(&self, other: &Bound) -> Option<Bound> {
        use std::cmp::{max, min};
        let lx = max(self.pos.x, other.pos.x);
        let uy = max(self.pos.y, other.pos.y);
        let rx = min(self.pos.x + self.dim.w, other.pos.x + other.dim.w);
        let ly = min(self.pos.y + self.dim.h, other.pos.y + other.dim.h);
        if (lx >= rx) || (uy >= ly) {
            None
        } else {
            Some(Bound {
                pos: Pos { x: lx, y: uy },
                dim: Dim {
                    w: rx - lx,
                    h: ly - uy,
                },
            })
        }
    }

    pub fn points(&self) -> Vec<Pos> {
        let mut v = Vec::with_capacity((self.dim.w * self.dim.h) as usize);
        for x in self.pos.x..self.pos.x + self.dim.w {
            for y in self.pos.y..self.pos.y + self.dim.h {
                v.push(Pos { x, y })
            }
        }
        return v;
    }
}

impl Claim {
    fn new(id: u16, x: u16, y: u16, w: u16, h: u16) -> Claim {
        Claim {
            id,
            bound: Bound {
                pos: Pos { x, y },
                dim: Dim { w, h },
            },
        }
    }
    fn conflict(&self, other: &Claim) -> Vec<Pos> {
        if self.id != other.id {
            self.bound
                .intersection(&other.bound)
                .map_or(Vec::new(), |x| x.points())
        } else {
            Vec::new()
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let re =
        regex::Regex::new(r"#(?P<id>\d+)\s+@\s+(?P<x>\d+),(?P<y>\d+):\s+(?P<w>\d+)x(?P<h>\d+)")
            .unwrap();
    re.captures_iter(input)
        .map(|c| {
            let f = |n| c.name(n).unwrap().as_str().parse::<u16>().unwrap();
            Claim::new(f("id"), f("x"), f("y"), f("w"), f("h"))
        })
        .collect()
}

#[aoc(day3, part1, fabric)]
pub fn fabric(input: &[Claim]) -> u32 {
    let mut fabric = std::collections::HashMap::new();
    for p in input.iter().flat_map(|x| x.bound.points()){
        *fabric.entry(p).or_insert(0) += 1;
    }
    fabric.iter().filter_map(|x| if *x.1 > 1 { Some(1) } else { None }).sum()
}

#[aoc(day3, part1, direct_conflicts)]
pub fn contested(input: &[Claim]) -> usize {
    let mut contested_points = std::collections::HashSet::new();
    let mut claim_iter = input.iter();
    while let Some(claim1) = claim_iter.next() {
        for claim2 in claim_iter.clone() {
            for point in claim1.conflict(claim2) {
                contested_points.insert(point);
            }
        }
    }
    contested_points.len()
}

#[aoc(day3, part2, uncontested)]
pub fn uncontested(input: &[Claim]) -> u16 {
    let mut all_claims = std::collections::HashSet::new();
    let mut contested_claims = std::collections::HashSet::new();
    let mut claim_iter = input.iter();
    while let Some(claim1) = claim_iter.next() {
        all_claims.insert(claim1.id);
        for claim2 in claim_iter.clone() {
            if  claim1.conflict(claim2).len() > 0 {
                contested_claims.insert(claim1.id);
                contested_claims.insert(claim2.id);
            }
        }
    }
    *all_claims.difference(&contested_claims).next().unwrap()
}
