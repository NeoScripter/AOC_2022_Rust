//To solve part 1, just change and number of array elements to 2 and "if head_idx == 8" to "if head_idx == 1" in the move_tail() method

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Rope {
    nodes: [(i32, i32); 10],
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            nodes: [(0, 0); 10],
            visited: HashSet::new(),
        }
    }
    fn move_head(&mut self, line: &str) {
        let (dir, dist) = line.split_once(' ').unwrap();
        for _ in 0..dist.parse::<usize>().unwrap() {
            match dir {
                "R" => self.nodes[0].1 += 1,
                "L" => self.nodes[0].1 -= 1,
                "U" => self.nodes[0].0 += 1,
                "D" => self.nodes[0].0 -= 1,
                _ => println!("Invalid input"),
            }
            for idx in 0..self.nodes.len() - 1 {
                self.move_tail(idx);
            }
        }
    }
    fn move_tail(&mut self, head_idx: usize) {
        let head = self.nodes[head_idx];
        let tail = &mut self.nodes[head_idx + 1];
    
        let diff_x = head.0.abs_diff(tail.0);
        let diff_y = head.1.abs_diff(tail.1);
    
        if head.0 == tail.0 && diff_y > 1 {
            tail.1 += (head.1 - tail.1).signum();
        } else if head.1 == tail.1 && diff_x > 1 {
            tail.0 += (head.0 - tail.0).signum();
        } else if diff_x > 1 || diff_y > 1 {
            tail.1 += (head.1 - tail.1).signum();
            tail.0 += (head.0 - tail.0).signum();
        }
    
        if head_idx == 8 {
            self.visited.insert(*tail);
        }
    }
    
    fn visited_once(&self) -> usize {
        self.visited.len()
    }
}

fn solve() -> usize {
    let mut rope = Rope::new();
    let input = include_str!("input9.txt");
    input.lines().for_each(|line| rope.move_head(line));
    rope.visited_once()
}

fn main() {
    println!("{}", solve());
}
