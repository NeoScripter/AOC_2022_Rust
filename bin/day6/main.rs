use itertools::Itertools;

fn part1(input: &str) -> usize {
    input.as_bytes().windows(4).position(|win| win.iter().all_unique()).unwrap() + 4
}

fn part2(input: &str) -> usize {
    input.as_bytes().windows(14).position(|win| win.iter().all_unique()).unwrap() + 14
}
fn main() {
    let input = include_str!("input6.txt");
    println!("{}", part2(input));
}