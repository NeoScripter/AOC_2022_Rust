use std::fmt;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cave {
    cave: Vec<Vec<char>>,
    cmds: Vec<Vec<(usize, usize)>>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cave {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Cave {
    fn new(x: usize, y: usize, cmds: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            cave: vec![vec!['.'; x]; y],
            cmds,
        }
    }
    fn populate(&mut self) {
        self.cmds.iter().for_each(|line| {
            line.iter().tuple_windows().for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    let (st_y, end_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                    for new_y in *st_y..=*end_y { self.cave[new_y][*x1] = '#' }
                } else {
                    let (st_x, end_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                    for new_x in *st_x..=*end_x { self.cave[*y1][new_x] = '#' }
                }
            })
        })
    }
    fn simulate(&mut self, st_x: usize, max_y: usize, part2: bool) -> u32 {
        for n in 0.. {
            let (mut x, mut y) = (st_x, 0);
            if self.cave[y][x] == 'O' && part2 { return n }
            loop {
                if y + 1 == max_y && !part2 { return n }
                if self.cave[y + 1][x] == '.' { y += 1 }
                else if self.cave[y + 1][x - 1] == '.' { y += 1; x -= 1 }
                else if self.cave[y + 1][x + 1] == '.' { y += 1; x += 1 }
                else { self.cave[y][x] = 'O'; break }
            }
        }
        0
    }
}

fn parse() -> (Cave, usize, usize) {
    let input = include_str!("input14.txt");

    let (mut max_x, mut max_y) = (0, 0);

    let grid = input.lines().map(|line| {
        line.split(" -> ").map(|pair| {
            let crd: Vec<usize> = pair.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            max_x = max_x.max(crd[0]);
            max_y = max_y.max(crd[1]);
            (crd[0], crd[1])
        }).collect::<Vec<(usize, usize)>>()
    }).collect();
    let cave = Cave::new(max_x + 500, max_y + 2, grid);
    (cave, max_x, max_y)
}

fn part1() -> u32 {
    let (mut cave, max_x, max_y) = parse();
    cave.populate();
    cave.simulate(500, max_y + 2, false)
}

fn part2() -> u32 {
    let (mut cave, max_x, max_y) = parse();
    cave.cave.push(vec!['#'; max_x + 500]);
    cave.populate();
    cave.simulate(500, max_y + 1, true)
}
fn main() {
    println!("{}", part1());
}