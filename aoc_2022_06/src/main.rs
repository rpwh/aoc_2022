use std::collections::HashSet;

fn main() {
    //let marker_size = 4;
    let marker_size = 14;
    let idx = std::fs::read_to_string("input.txt").unwrap()
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_size)
        .enumerate()
        .find(|(_, x)| HashSet::<&char>::from_iter(x.iter()).len() == marker_size)
        .unwrap().0;

    println!("{}", idx+marker_size);
}
