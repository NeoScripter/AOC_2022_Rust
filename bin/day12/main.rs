use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

#[derive(Debug)]
struct Heightmap {
    map: Vec<Vec<u8>>,
    st_pts: Vec<(usize, usize)>,
    end: (usize, usize),
}

impl Heightmap {
    fn find_path(&self) -> u32 {
        let mut cache = HashSet::new();
        let mut queue = BinaryHeap::new();
        self.st_pts.iter().for_each(|&point| queue.push((Reverse(0), point)));
        while let Some((Reverse(steps), (y, x))) = queue.pop() {
            if !cache.insert((y, x)) {continue}
            if (y, x) == self.end {return steps}

            if y > 0 {if self.map[y - 1][x] - 1 <= self.map[y][x] {
                    queue.push((Reverse(steps + 1), (y - 1, x)))}
            }
            if y < self.map.len() - 1 {if self.map[y + 1][x] - 1 <= self.map[y][x] {
                    queue.push((Reverse(steps + 1), (y + 1, x)))}
            }
            if x > 0 {if self.map[y][x - 1] - 1 <= self.map[y][x] {
                    queue.push((Reverse(steps + 1), (y, x - 1)))}
            }
            if x < self.map[0].len() - 1 {if self.map[y][x + 1] - 1 <= self.map[y][x] {
                    queue.push((Reverse(steps + 1), (y, x + 1)))}
            }
        }
        0
    }
}

fn solve() -> u32 {
    let input = include_str!("input12.txt");
    let mut st_pts = Vec::new();
    let mut end = (0, 0);
    let grid: Vec<Vec<u8>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                'S' | 'a' => { st_pts.push((y, x)); 1 },
                'E' => { end = (y, x); 26 },
                _ => (c as u8) - b'a' + 1
            }
        }).collect()
    }).collect();
    let hgtmap = Heightmap { map: grid, st_pts, end };

    hgtmap.find_path()
}

fn main() {
    println!("{}", solve());
}