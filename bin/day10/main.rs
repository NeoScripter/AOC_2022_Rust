trait Cycle {
    fn cycle(&mut self, line: &str);
}

#[derive(Debug, Clone)]
struct CRT {
    rgtr: i32,
    cycle: i32,
    output: String,
}

impl CRT {
    fn new() -> Self {
        Self {
            rgtr: 1,
            cycle: 0,
            output: String::new(),
        }
    }
    fn is_visible(&self) -> bool {
        (-1..=1).any(|n| self.rgtr + n == self.cycle % 40)
    }
    fn new_line(&self) -> bool {
        (self.cycle + 1) % 40 == 0
    }
    fn update(&mut self) {
        self.output.push(if self.is_visible() { '#' } else { '.' });
        if self.new_line() {self.output.push_str("\r\n")}
    }
}

impl Cycle for CRT {
    fn cycle(&mut self, line: &str) {
        self.update();
        if line.starts_with("addx") {
            self.cycle += 1;
            self.update();
            let (_, value) = line.split_once(' ').unwrap();
            self.cycle += 1;
            self.rgtr += value.parse::<i32>().unwrap();
        }
        if line.starts_with("noop") {self.cycle += 1}
    }
}

#[derive(Debug, Clone)]
struct CPU {
    rgtr: i32,
    cycle: i32,
    output: Vec<i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            rgtr: 1,
            cycle: 0,
            output: Vec::new(),
        }
    }
    fn find_signal_str(&self) -> i32 {
        self.output.iter().take(6).sum()
    }
    fn update(&mut self) {
        if (self.cycle - 20) % 40 == 0 {self.output.push(self.rgtr * self.cycle)}
    }
}

impl Cycle for CPU {
    fn cycle(&mut self, line: &str) {
        if line.starts_with("addx") {
            self.cycle += 1;
            self.update();
            let (_, value) = line.split_once(' ').unwrap();
            self.cycle += 1;
            self.update();
             self.rgtr += value.parse::<i32>().unwrap();
        }
        if line.starts_with("noop") {
            self.cycle += 1;
            self.update();
        }
    }
}

fn cycle<T: Cycle>(device: &mut T, line: &str) {
    device.cycle(line);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input10.txt");
    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    input.lines().for_each(|line| {
        cycle(&mut cpu, line);
        cycle(&mut crt, line);
    });
    println!("{}", cpu.find_signal_str());
    println!("{}", crt.output);
    Ok(())
}