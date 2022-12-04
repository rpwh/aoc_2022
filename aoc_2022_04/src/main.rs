fn main() {
    let p1 = part_one();
    let p2 = part_two();
    
    println!("{p1}:{p2}");
}

fn part_one() -> u32 {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            return line
                .replace(','," ")
                .replace('-', " ")
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .chunks_exact(4)
                .map(|c| {
                    let sec_a = c[0]..=c[1];
                    let sec_b = c[2]..=c[3];
                    
                    if sec_a.contains(&c[2]) && sec_a.contains(&c[3]) || sec_b.contains(&c[0]) && sec_b.contains(&c[1]) {
                        1
                    } else {
                        0
                    }
                })
                .next()
                .unwrap();
            
        })
        .sum()
}

fn part_two() -> u32 {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            return line
                .replace(','," ")
                .replace('-', " ")
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .chunks_exact(4)
                .map(|c| {
                    let mut sec_a = c[0]..=c[1];
                    let sec_b = c[2]..=c[3];

                    if sec_a.any(|x| sec_b.contains(&x)){
                        1
                    } else {
                        0
                    }
                })
                .next()
                .unwrap();
            
        })
        .sum()

}
