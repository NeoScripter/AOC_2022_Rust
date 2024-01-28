use nom::{
    IResult,
    bytes::complete::tag,
    sequence::{tuple, preceded},
    character::complete::digit1,
    combinator::{map_res, map, recognize, opt},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Map {
    sensors: Vec<(i64, i64, i64)>,
    zones: HashMap<i64, Vec<(i64, i64)>>,
}

impl Map {
    fn new(sensors: Vec<(i64, i64, i64)>) -> Self {
        Self { sensors, zones: HashMap::new() }
    }
    fn find_zones(&mut self, st: i64, end: i64) {
        (st..=end).for_each(|row| {
            let mut cvrd = Vec::new();
            for &(x, y, d) in self.sensors.iter() {
                if y >= row - d && y <= row + d {
                    let delta = d - (y - row).abs();
                    cvrd.push((x - delta, x + delta));
                }
            }
            cvrd.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            let merged = merge(cvrd);
            self.zones.insert(row, merged);
        })
    }
    fn find_exluded(&self, row: i64) -> u64 {
        self.zones[&row].iter().map(|&(st, end)| st.abs_diff(end)).sum()
    }
    fn find_beacon(&self, lmt: i64) -> (i64, i64) {
        for y in 0..=lmt {
            if let Some(ranges) = self.zones.get(&y) {
                let mut x = 0;
                for &(st, end) in ranges {
                    if x < st { return (y, x); }
                    x = end.max(x) + 1;
                    if x > lmt { break; }
                }
                if x <= lmt { return (y, x); }
            }
        }
        (0, 0)
    }
}

fn merge(cvrd: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for zone in cvrd {
        if let Some(last) = merged.last_mut() {
            if last.1 >= zone.0 {
                last.1 = last.1.max(zone.1);
                continue;
            }
        }
        merged.push(zone);
    }
    merged
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        str::parse::<i64>(s)
    })(input)
}

fn line_parser(input: &str) -> IResult<&str, (i64, i64, i64, i64)> {
    map(tuple((tag("Sensor at x="), parse_i64, tag(", y="), parse_i64, tag(": closest beacon is at x="), parse_i64, tag(", y="), parse_i64)),|(_, sx, _, sy, _, bx, _, by)| {
            (sx, sy, bx, by)
        },
    )(input)
}

fn parse() -> Map {
    let input = include_str!("input15.txt");
    let sensors: Vec<(i64, i64, i64)> = input.lines().map(|l| {
        let (_, (sx, sy, bx, by)) = line_parser(l).unwrap();
        let d = (sx - bx).abs() + (sy - by).abs();
        (sx, sy, d)
    }).collect();
    let map = Map::new(sensors);
    map
}

fn part1(row: i64) -> u64 {
    let mut map = parse();
    map.find_zones(row, row);
    map.find_exluded(row)
}

fn part2(lmt: i64) -> i64 {
    let mut map = parse();
    map.find_zones(0, lmt);
    let (y, x) = map.find_beacon(lmt);
    x * 4000000 + y
}

fn main() {
    println!("{}", part2(4000000));
}