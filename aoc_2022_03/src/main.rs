fn main() {
    let p1 = part_one();
    let p2 = part_two();
    println!("{p1}:{p2}");
}

fn part_one() -> u32 {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let (left, right) = s.as_bytes().split_at(s.len()/2);
            let &common = left.iter().find(|c| right.contains(c)).unwrap();
            if common >= b'a' {
                (common + 1 - b'a') as u32
            } else {
                (common + 27 -b'A') as u32
            }
        })
        .sum()
}


fn part_two() -> u32 {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.as_bytes()).collect::<Vec<&[u8]>>()
        .chunks(3)
        .map(|elf| {
            let &common = elf[0]
                .iter()
                .find(|c| elf[1].contains(c) && elf[2].contains(c))
                .unwrap();
            if common >= b'a' {
                (common + 1 - b'a') as u32
            } else {
                (common + 27 -b'A') as u32
            }
        })
        .sum()
}
