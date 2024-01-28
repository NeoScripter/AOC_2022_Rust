use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(u64),
    Add(u64),
    Pow,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected: u64,
}
impl Monkey {
    fn new(items: VecDeque<u64>, op: Op, test: u64, if_true: usize, if_false: usize) -> Self {
        Monkey {
            items,
            op,
            test,
            if_true,
            if_false,
            inspected: 0,
        }
    }
    fn adjust_worry(&self, worry: u64) -> u64 {
        match self.op {
            Op::Mul(x) => worry * x,
            Op::Add(x) => worry + x,
            Op::Pow => worry * worry,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkeys (Vec<Monkey>);

impl Monkeys {
    fn turn(&mut self, idx: usize, d: u64) {
        let mnk = &mut self.0[idx];
        let mut items = Vec::new();
        while let Some(item) = mnk.items.pop_front() {
            let mut new_item = mnk.adjust_worry(item);
            if d == 3 {new_item /= d} else {new_item %= d}
            mnk.inspected += 1;
            let tg_idx = if new_item % mnk.test == 0 {mnk.if_true} else {mnk.if_false};
            items.push((new_item, tg_idx));
        }
        items.reverse();
        while let Some((item, idx)) = items.pop() {
            self.0[idx].items.push_back(item)
        }
    }
    fn round(&mut self, d: u64) {
        (0..self.0.len()).for_each(|id| self.turn(id, d))
    }
    fn monkey_business(&self) -> u64 {
        self.0.iter().map(|mnk| mnk.inspected).sorted_unstable_by(|a, b| b.cmp(a)).take(2).product()
    }
}

fn parse_input() -> Option<Monkeys> {
    let input = include_str!("input11.txt");
    let mut mnks = Vec::new();
    for monkey in input.split("\r\n\r\n") {
        let mut iter = monkey.lines().skip(1);
        let items = iter.next()?.trim().strip_prefix("Starting items: ")?;
        let itms: VecDeque<u64> = items.split(", ").filter_map(|x| x.parse().ok()).collect();
        let operation = iter.next()?.trim().strip_prefix("Operation: new = old ")?;
        let (sign, num) = operation.split_once(' ')?;
        let op = match num.parse::<u64>().ok() {
            Some(num) => match sign {
                "*" => Op::Mul(num),
                _ => Op::Add(num),
            },
            None => Op::Pow,
        };
        let (_, test) = iter.next()?.rsplit_once(' ')?;
        let test = test.parse::<u64>().ok()?;
        let (_, num) = iter.next()?.rsplit_once(' ')?;
        let if_true = num.parse::<usize>().ok()?;
        let (_, num) = iter.next()?.rsplit_once(' ')?;
        let if_false = num.parse::<usize>().ok()?;
        let mnk = Monkey::new(itms, op, test, if_true, if_false);
        mnks.push(mnk);
    }
    Some(Monkeys(mnks))
}

fn part1() -> u64 {
    let mut mnks = parse_input().unwrap();
    for _ in 0..20 {mnks.round(3)}
    mnks.monkey_business()
}
fn part2() -> u64 {
    let mut mnks = parse_input().unwrap();
    let d: u64 = mnks.0.iter()
        .map(|mnk| mnk.test)
        .product();
    for _ in 0..10000 {mnks.round(d)}
    mnks.monkey_business()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("{}", part2());
    Ok(())
}