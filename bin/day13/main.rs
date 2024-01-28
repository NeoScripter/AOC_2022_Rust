use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse_line(s: &str) -> Self {
        let mut iter = s.chars();
        Self::parse_list(&mut iter)
    }
    fn parse_list(iter: &mut std::str::Chars<'_>) -> Self {
        let mut new_list = Vec::new();
        let mut num = String::new();

        while let Some(next) = iter.next() {
            match next {
                '[' => new_list.push(Self::parse_list(iter)),
                ',' => {if !num.is_empty() {
                        new_list.push(Packet::Num(num.parse::<u32>().unwrap()));
                        num.clear()}},
                ']' => {
                    if !num.is_empty() {new_list.push(Packet::Num(num.parse::<u32>().unwrap()))}
                    break},
                _ => num.push(next),
            }
        }
        Packet::List(new_list)
    }
}

fn parse_input() -> Vec<[Packet; 2]> {
    let input = include_str!("input13.txt");
    input.split("\r\n\r\n").map(|pair| {
        let (left, right) = pair.split_once("\r\n").unwrap();
        [Packet::parse_line(left), Packet::parse_line(right)]
    }).collect()
}

fn part1() -> usize {
    let pairs = parse_input();
    
    pairs
    .iter()
    .positions(|[left, right]| left < right)
    .map(|idx| idx + 1)
    .sum()
}

fn part2() -> usize {
    let pairs = parse_input();
    let new_pks = [Packet::parse_line("[[2]]"), Packet::parse_line("[[6]]")];

    let mut pairs: Vec<_> = pairs.into_iter().flatten().collect();

    pairs.extend(new_pks.clone());
    pairs.sort_unstable();

    pairs
    .into_iter()
    .positions(|packet| new_pks.contains(&packet))
    .map(|idx| idx + 1)
    .product()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("{}", part2());
    Ok(())
}