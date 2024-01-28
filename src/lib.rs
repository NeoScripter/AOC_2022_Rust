#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]

use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque, BinaryHeap},
};
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    sequence::{tuple, terminated},
    character::complete::{alpha1, digit1, space0},
    combinator::{map_res, map, opt, recognize},
    multi::separated_list1,
    branch::alt,
};

fn until_valve_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_uppercase())(input)
}

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_list(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(s)
}

fn parse_line(input: &str) -> IResult<&str, (&str, u32, HashSet<&str>)> {
    map(tuple((tag("Valve "), alpha1, tag(" has flow rate="), parse_integer, until_valve_name, parse_list)),|(_, name, _, flow_rate, _, adj_tunnels)| {
            (name, flow_rate, adj_tunnels.into_iter().collect())
        },
    )(input)
}

#[derive(Debug, Clone)]
struct State<'a> {
    elapsed: u32,
    relieved: u32,
    name: &'a str,
    open: BTreeSet<&'a str>,
}

impl<'a> State<'a> {
    fn new(elapsed: u32, relieved: u32, name: &'a str, open: BTreeSet<&'a str>) -> Self {
        Self { elapsed, relieved, name, open }
    }
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    flow: u32,
    nghs: HashSet<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(flow: u32, nghs: HashSet<&'a str>) -> Self {
        Self { flow, nghs }
    }
}

#[derive(Debug, Clone, Eq)]
struct Node<'a> {
    cost: u32,
    name: &'a str,
}

impl<'a> Node<'a> {
    fn new(cost: u32, name: &'a str) -> Self {
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
    min_dist: HashMap<(&'a str, &'a str), u32>,
}

impl <'a>Graph<'a> {
    fn new() -> Self { 
        Self {
            valves: HashMap::new(),
            min_dist: HashMap::new(),
        }
    }
    fn find_cost(&self, from: &'a str, to: &'a str) -> u32 {
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
            *acc.entry(("AA", n1)).or_insert_with(|| self.find_cost("AA", n1));
            *acc.entry(("AA", n2)).or_insert_with(|| self.find_cost("AA", n2));
            acc.insert((n1, n2), self.find_cost(n1, n2));
            acc.insert((n2, n1), self.find_cost(n2, n1));
            acc
        });
        self.min_dist = map;
    }
    fn accelerate_end(&self, max_time: u32, elapsed: u32, relieved: u32, open: &BTreeSet<&'a str>) -> u32 {
        let time_left = max_time - elapsed;
        let per_minute: u32 = open.iter().map(|n| self.valves[n].flow).sum();
        relieved + time_left * per_minute
    }
    fn find_path(&self) -> u32 {
        let flowing: HashSet<&str> = self.valves.iter().filter(|(_, v)| v.flow > 0).map(|(n, _)| *n).collect();
        let mut pq = VecDeque::new();
        let mut max_relieved_states: HashMap<BTreeSet<&str>, u32> = HashMap::new();
        pq.push_back(State::new(0, 0, "AA", BTreeSet::new()));
        while let Some(State {elapsed, relieved, name, open}) = pq.pop_front() {
            let relieved_at_end = self.accelerate_end(26, elapsed, relieved, &open);

            max_relieved_states.entry(open.clone()).and_modify(|val| *val = relieved_at_end.max(*val)).or_insert(relieved_at_end);

            if open.len() == flowing.len() || elapsed >= 26 {
                continue;
            }

            let closed = flowing.iter().filter(|n| !open.contains(*n));

            for &p in closed {
                let cost = self.min_dist[&(name, p)] + 1;
                let new_elapsed = elapsed + cost;
                if new_elapsed >= 26 { continue }

                let per_minute: u32 = open.iter().map(|n| self.valves[n].flow).sum();
                let new_relieved  = relieved + cost * per_minute;

                let mut new_open = open.clone();
                new_open.insert(p);

                pq.push_back(State::new(new_elapsed, new_relieved, p, new_open));
            }
        }
        max_relieved_states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap()
    }
}

fn part1() -> u32 {
    let input = include_str!("input_lib.txt");
    let mut graph = Graph::new();
    for line in input.lines() {
        let (_, (name, flow, nghs)) = parse_line(line).unwrap();
        graph.valves.insert(name, Valve::new(flow, nghs));
    }
    graph.find_dist();
    graph.find_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_output() {
        assert_eq!(1707, part1());
    }
    #[test]
    fn test_process_output2() {
        assert_eq!(56000011, part1());
    }
}