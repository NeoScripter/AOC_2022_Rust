
fn char_to_score(c: char) -> i32 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0,
    }
}

fn part1(input: &str) -> i32 {
    let mut score = 0;
    input.lines().for_each(|game| {
        let mut iter = game.chars();
        let op = char_to_score(iter.next().unwrap());
        let you = char_to_score(iter.skip(1).next().unwrap());
        match (op - you).rem_euclid(3) {
            2 => score += 6 + you,
            1 => score += you,
            _ => score += 3 + you,
        }
    });
    score
}

fn char_to_outcome(c: char, score: &mut i32) -> i32 {
    match c {
        'X' => 1,
        'Y' => {
            *score += 3;
            0
        },
        'Z' => {
            *score += 6;
            2
        },
        _ => 0,
    }
}

fn part2(input: &str) -> i32 {
    let mut score = 0;
    input.lines().for_each(|game| {
        let mut iter = game.chars();
        let op = char_to_score(iter.next().unwrap());
        let outcome = char_to_outcome(iter.skip(1).next().unwrap(), &mut score);
        match (op - outcome).rem_euclid(3) {
            0 => score += 3,
            1 => score += 1,
            _ => score += 2,
        }
    });
    score
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}", part2(input));
}