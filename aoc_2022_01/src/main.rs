use itertools::Itertools;

fn main() {
    let top_three: Vec<u32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|x| x.lines().filter_map(|x| x.parse::<u32>().ok()).sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .collect();
    
    println!("Part 1: {}, Part 2: {}", top_three[0], top_three.iter().sum::<u32>());
}
