use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day_1").context("Error reading input")?;

    let totals = input.split("\n\n").map(|s| {
        s.split_whitespace()
            .map(|f| f.parse::<usize>().expect("Error parsing string"))
            .sum::<usize>()
    });

    println!("{}", part_1(totals.clone())?);
    println!("{}", part_2(totals)?);

    Ok(())
}

fn part_1(totals: impl Iterator<Item = usize>) -> Result<usize> {
    totals.max().context("No maximum found")
}

fn part_2(totals: impl Iterator<Item = usize>) -> Result<usize> {
    let mut totals: Vec<_> = totals.collect();
    totals.sort_unstable();
    Ok(totals.iter().rev().take(3).sum())
}
