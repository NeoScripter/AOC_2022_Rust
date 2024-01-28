#[derive(Debug, Clone)]
struct Forest (Vec<Vec<u8>>);

impl Forest {
    fn check_right(&self, h: u8, x: usize, y: usize) -> (bool, usize) {
        if x < self.0[0].len() - 1 {
            if h > self.0[y][x + 1] {
                let (visible, score) = self.check_right(h, x + 1, y);
                (visible, score + 1)
            } else {
                (false, 1)
            }
        } else {(true, 0)}
    }
    fn check_left(&self, h: u8, x: usize, y: usize) -> (bool, usize) {
        if x > 0 {
            if h > self.0[y][x - 1] {                        
                let (visible, score) = self.check_left(h, x - 1, y);
                (visible, score + 1)
            } else {
                (false, 1)
            }
        } else {(true, 0)}
    }
    fn check_up(&self, h: u8, x: usize, y: usize) -> (bool, usize) {
        if y < self.0.len() - 1 {
            if h > self.0[y + 1][x] {
                let (visible, score) = self.check_up(h, x, y + 1);
                (visible, score + 1)
            } else {
                (false, 1)
            }
        } else {(true, 0)}
    }
    fn check_down(&self, h: u8, x: usize, y: usize) -> (bool, usize) {
        if y > 0 {
            if h > self.0[y - 1][x] {
                    let (visible, score) = self.check_down(h, x, y - 1);
                    (visible, score + 1)
                } else {
                    (false, 1)
                }
        } else {(true, 0)}
    }
    fn find_visible_trees(&self) -> usize {
        (0..self.0.len()).flat_map(|y| {
            (0..self.0[0].len()).filter(move |&x| {
                let h = self.0[y][x];
                self.check_right(h, x, y).0 || self.check_left(h, x, y).0 || self.check_up(h, x, y).0 || self.check_down(h, x, y).0
            })
        }).count()
    }
    fn find_scenic_score(&self) -> usize {
        (0..self.0.len()).flat_map(|y| {
            (0..self.0[0].len()).map(move |x| {
                let h = self.0[y][x];
                self.check_right(h, x, y).1 * self.check_left(h, x, y).1 * self.check_up(h, x, y).1 * self.check_down(h, x, y).1
            })
        }).max().unwrap()
    }
}

fn parse_input() -> Forest {
    let input = include_str!("input8.txt");
    Forest(input.lines()
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect())
}

fn part1() -> usize {
    let forest = parse_input();
    forest.find_visible_trees()
}

fn part2() -> usize {
    let forest = parse_input();
    forest.find_scenic_score()
}

fn main() {
    println!("{}", part2());
}