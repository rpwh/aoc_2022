const WIN: [u32; 3] = [1,2,0];
const DIFF: [u32; 3] = [2,0,1];

fn main() {
    let p1 = parse_input()
        .into_iter()
        .fold(0, |acc, (opp, me)| acc + me + 1 + points(opp, me));

    let p2 = parse_input()
        .into_iter()
        .map(|(opp, me)| (opp, (opp + DIFF[me as usize]) % 3))
        .fold(0, |acc, (opp, me)| acc + me + 1 + points(opp, me));

    println!("Part 1: {p1}, Part 2: {p2}");
}

fn parse_input() -> Vec<(u32, u32)> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| ((x.as_bytes()[0]-b'A') as u32, (x.as_bytes()[2]-b'X') as u32))
        .collect()
}

fn points(opp: u32, me: u32) -> u32 {
    if opp == me {return 3}
    if WIN[opp as usize] == me {return 6} 
    0
}
