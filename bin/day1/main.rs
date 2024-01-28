use itertools::Itertools;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let mut result = include_str!("input1.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    .map(|elf| {
        elf.lines()
            .filter_map(|s| s.parse::<u32>().ok())
            .sum::<u32>()
    })
    .sorted()
    .rev()
    .take(3)
    .sum::<u32>();

    println!("{}", result);

    Ok(())
}

