use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque, BinaryHeap};
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    sequence::tuple,
    character::complete::{alpha1, digit1},
    combinator::{map_res, map},
    multi::separated_list1,
};

fn until_valve_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_uppercase())(input)
}

fn parse_integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_list(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(s)
}

fn parse_line(input: &str) -> IResult<&str, (&str, u64, HashSet<&str>)> {
    map(tuple((tag("Valve "), alpha1, tag(" has flow rate="), parse_integer, until_valve_name, parse_list)),|(_, name, _, flow_rate, _, adj_tunnels)| {
            (name, flow_rate, adj_tunnels.into_iter().collect())
        },
    )(input)
}

#[derive(Debug, Clone)]
struct State<'a> {
    elapsed: u64,
    relieved: u64,
    name: &'a str,
    open: BTreeSet<&'a str>,
}

impl<'a> State<'a> {
    fn new(elapsed: u64, relieved: u64, name: &'a str, open: BTreeSet<&'a str>) -> Self {
        Self { elapsed, relieved, name, open }
    }
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    flow: u64,
    nghs: HashSet<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(flow: u64, nghs: HashSet<&'a str>) -> Self {
        Self { flow, nghs }
    }
}

#[derive(Debug, Clone, Eq)]
struct Node<'a> {
    cost: u64,
    name: &'a str,
}

impl<'a> Node<'a> {
    fn new(cost: u64, name: &'a str) -> Self {
        Self { cost, name }
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
    min_dist: HashMap<(&'a str, &'a str), u64>,
}

impl <'a>Graph<'a> {
    fn new() -> Self { 
        Self {
            valves: HashMap::new(),
            min_dist: HashMap::new(),
        }
    }
    fn find_cost(&self, from: &'a str, to: &'a str) -> u64 {
        let mut cache = HashSet::new();
        let mut pq = BinaryHeap::new();
        pq.push(Node::new(0, from));
        while let Some(Node { cost, name }) = pq.pop() {
            if name == to { return cost }
            if !cache.insert(name) { continue }

            for n in &self.valves[&name].nghs {
                pq.push(Node::new(cost + 1, n));
            }
        }
        unreachable!()
    }
    fn find_dist(&mut self) {
        let map = self.valves.iter().filter(|(_, v)| v.flow > 0).map(|(&n, _)| n).tuple_combinations().fold(HashMap::new(), |mut acc, (n1, n2)| {
            acc.entry(("AA", n1)).or_insert_with(|| self.find_cost("AA", n1));
            acc.entry(("AA", n2)).or_insert_with(|| self.find_cost("AA", n2));
            acc.insert((n1, n2), self.find_cost(n1, n2));
            acc.insert((n2, n1), self.find_cost(n2, n1));
            acc
        });
        self.min_dist = map;
    }
    fn accelerate_end(&self, max_time: u64, elapsed: u64, relieved: u64, open: &BTreeSet<&'a str>) -> u64 {
        let time_left = max_time - elapsed;
        let per_minute: u64 = open.iter().map(|n| self.valves[n].flow).sum();
        relieved + time_left * per_minute
    }
    fn find_path1(&self) -> u64 {
        let flow: HashSet<&str> = self.valves.iter().filter(|(_, v)| v.flow > 0).map(|(n, _)| *n).collect();
        let mut cache = HashSet::new();
        let mut q = VecDeque::new();
        let mut max_pr = 0;
        q.push_back(State::new(0, 0, "AA", BTreeSet::new()));
        while let Some(State {elapsed: el, relieved: rel, name: nm, open: op}) = q.pop_front() {
            cache.insert((BTreeSet::new(), 0, 0));
    
            if op.len() == flow.len() || el >= 30 {
                let tot = self.accelerate_end(30, el, rel, &op);
                max_pr = max_pr.max(tot);
                continue;
            }
    
            let closed = flow.iter().filter(|n| !op.contains(*n));
    
            for &p in closed {
                let cost = self.min_dist[&(nm, p)] + 1;
                let new_el = el + cost;
                if new_el >= 30 {
                    let tot = self.accelerate_end(30, el, rel, &op);
                    max_pr = max_pr.max(tot);
                    continue;
                }
    
                let pm: u64 = op.iter().map(|n| self.valves[n].flow).sum();
                let new_rel = rel + cost * pm;
                let mut new_op = op.clone();
                new_op.insert(p);
    
                if cache.insert((new_op.clone(), new_el, new_rel)) {
                    q.push_back(State::new(new_el, new_rel, p, new_op));
                }
            }
        }
        max_pr
    }    
    fn find_path2(&self) -> u64 {
        let flow: HashSet<&str> = self.valves.iter().filter(|(_, v)| v.flow > 0).map(|(n, _)| *n).collect();
        let mut q = VecDeque::new();
        let mut max_relief: HashMap<BTreeSet<&str>, u64> = HashMap::new();
        q.push_back(State::new(0, 0, "AA", BTreeSet::new()));
        while let Some(State {elapsed: el, relieved: rel, name: nm, open: op}) = q.pop_front() {
            let rel_end = self.accelerate_end(26, el, rel, &op);
    
            max_relief.entry(op.clone()).and_modify(|v| *v = rel_end.max(*v)).or_insert(rel_end);
    
            if op.len() == flow.len() || el >= 26 { continue }
    
            let closed = flow.iter().filter(|n| !op.contains(*n));
    
            for &p in closed {
                let cost = self.min_dist[&(nm, p)] + 1;
                let new_el = el + cost;
                if new_el >= 26 { continue }
    
                let pm: u64 = op.iter().map(|n| self.valves[n].flow).sum();
                let new_rel = rel + cost * pm;
    
                let mut new_op = op.clone();
                new_op.insert(p);
    
                q.push_back(State::new(new_el, new_rel, p, new_op));
            }
        }
        max_relief
        .iter()
        .tuple_combinations()
        .filter(|(h, e)| h.0.is_disjoint(e.0))
        .map(|(h, e)| h.1 + e.1)
        .max()
        .unwrap()
    }    
}

fn solve() -> u64 {
    let input = include_str!("input16.txt");
    let mut graph = Graph::new();
    for line in input.lines() {
        let (_, (name, flow, nghs)) = parse_line(line).unwrap();
        graph.valves.insert(name, Valve::new(flow, nghs));
    }
    graph.find_dist();
    graph.find_path1()
}

fn main() {
    println!("{}", solve());
}