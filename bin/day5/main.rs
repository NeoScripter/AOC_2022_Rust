use std::collections::{BTreeMap, VecDeque};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, opt},
    sequence::{delimited, preceded},
    IResult,
};

fn parse_crate(i: &str) -> IResult<&str, char> {
    let first_char = |s: &str| s.chars().next().unwrap();
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<char>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<char>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

#[derive(Debug, Clone)]
struct Crane {
    crts: BTreeMap<usize, Vec<char>>,
    cmds: VecDeque<Vec<usize>>,
}

impl Crane {
    fn new() -> Self {
        Self {
            crts: BTreeMap::new(),
            cmds: VecDeque::new(),
        }
    }
    fn move_crates_p1(&mut self) {
        while let Some(cmd) = self.cmds.pop_front() {
            let (qt, fr, to) = (cmd[0], cmd[1], cmd[2]);
            (0..qt).for_each(|_| {
                if let Some(item) = self.crts.get_mut(&fr).and_then(|from| from.pop()) {
                    self.crts.get_mut(&to).unwrap().push(item);
                }
            })
        }
    }
    fn move_crates_p2(&mut self) {
        while let Some(cmd) = self.cmds.pop_front() {
            let (qt, fr, to) = (cmd[0], cmd[1], cmd[2]);
            let mut temp = Vec::new();
            (0..qt).for_each(|_| {
                if let Some(item) = self.crts.get_mut(&fr).and_then(|from| from.pop()) {
                    temp.push(item);
                }
            });
            temp.reverse();
            self.crts.get_mut(&to).unwrap().extend(temp);
        }
    }
    fn final_cfg(&mut self) -> String {
        self.crts.values().filter_map(|v| v.last()).collect::<String>()
    }
}
fn solve(input: &str) -> String {
    let (crts, cmds) = input.split_once("\r\n\r\n").unwrap();
    let mut crane = Crane::new();
    crts.lines().rev().skip(1).for_each(|line| {
        if let Ok((_, row)) = parse_crate_line(line) {
            row.into_iter().enumerate().for_each(|(idx, c)| {
                match c {
                    Some(ch) => crane.crts.entry(idx + 1).or_insert_with(Vec::new).push(ch),
                    None => {},
                }
    })
        }
    });
    cmds.lines().for_each(|line| {
        let cmd_ln: Vec<usize> = line.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect();
        crane.cmds.push_back(cmd_ln)
    });
    crane.move_crates_p2();
    crane.final_cfg()
}
fn main() {
    let input = include_str!("input5.txt");
    println!("{}", solve(input));
}