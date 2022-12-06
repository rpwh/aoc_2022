use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let count = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap()-1;
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap()-1;
        Ok(Move { count, from, to})
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (crates, ops) = input.split_once("\r\n\r\n").unwrap();
    
    //Parse Crates into vectors
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    crates.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| *item != ' ')
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec = Vec::with_capacity(10);
                    new_vec.push(item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(item);
                }
            })
    });

    //Parse Moves
    let moves: Vec<Move> = ops.lines().flat_map(|x| x.parse().ok()).collect();
    
    //let p1 = part_one(&mut stacks, &moves);
    let p2 = part_two(&mut stacks, &moves);

    println!("{p2}");
}

fn part_one(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for m in moves {
        for _ in 0..m.count {
            let item = stacks[m.from].pop().unwrap();
            stacks[m.to].push(item);
        }
    }

    stacks.into_iter().map(|s| s.pop().unwrap()).collect()
}

fn part_two(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for m in moves {
        let to_height = stacks[m.to].len();
        for _ in 0..m.count {
            let item = stacks[m.from].pop().unwrap();
            stacks[m.to].insert(to_height, item);
        }
    }

    stacks.into_iter().map(|s| s.pop().unwrap()).collect()
}
