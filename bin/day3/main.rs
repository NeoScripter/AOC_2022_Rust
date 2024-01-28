use std::collections::HashMap;
use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let prs: HashMap<char, i32> = (('a'..='z').chain('A'..='Z')).zip(1..=52).collect();
    input.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let matched: char = left.chars().find(|&x| right.contains(x)).unwrap();
        prs.get(&matched).unwrap()
    }).sum()
}

fn part2(input: &str) -> i32 {
    let prs: HashMap<char, i32> = (('a'..='z').chain('A'..='Z')).zip(1..=52).collect();
    input.lines().tuples::<(_, _, _)>().map(|(l1, l2, l3)| {
        let matched: char = l1.chars().find(|&x| l2.contains(x) && l3.contains(x)).unwrap();
        prs.get(&matched).unwrap()
    }).sum()
}

fn main() {
    let input = include_str!("input3.txt");
    println!("{}", part2(input));
}
