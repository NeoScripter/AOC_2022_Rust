
fn part1(input: &str) -> usize {
    input.lines().filter(|&line| {
        let r: Vec<u8> = line.split(|c| c == '-' || c == ',').map(|x| x.parse().unwrap()).collect();
        let range1 = r[0]..=r[1];
        let range2 = r[2]..=r[3];

        range1.clone().all(|x| range2.contains(&x)) || range2.clone().all(|x| range1.contains(&x))
    }).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|&line| {
        let r: Vec<u8> = line.split(|c| c == '-' || c == ',').map(|x| x.parse().unwrap()).collect();
        let range1 = r[0]..=r[1];
        let range2 = r[2]..=r[3];

        range1.clone().any(|x| range2.contains(&x))
    }).count()
}

fn main() {
    let input = include_str!("input4.txt");
    println!("{}", part2(input));
}